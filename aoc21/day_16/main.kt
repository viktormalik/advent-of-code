import java.io.File

data class Packet(val version: Int, val id: Int) {
    var value: Long = 0
    val innerPackets = mutableListOf<Packet>()
    var len = 0

    fun versionSum(): Int = version + innerPackets.map { it.versionSum() }.sum()

    fun eval(): Long = when (id) {
        0 -> innerPackets.map { it.eval() }.sum()
        1 -> innerPackets.fold(1) { prod, p -> prod * p.eval() }
        2 -> innerPackets.map { it.eval() }.minOrNull()!!
        3 -> innerPackets.map { it.eval() }.maxOrNull()!!
        4 -> value
        5 -> if (innerPackets[0].eval() > innerPackets[1].eval()) 1 else 0
        6 -> if (innerPackets[0].eval() < innerPackets[1].eval()) 1 else 0
        7 -> if (innerPackets[0].eval() == innerPackets[1].eval()) 1 else 0
        else -> 0
    }
}

fun parse(str: String): Packet {
    val version = str.substring(0, 3).toInt(radix = 2)
    val id = str.substring(3, 6).toInt(radix = 2)
    val packet = Packet(version, id)

    var headerLen = 6
    var contentLen = 0

    if (id == 4) {
        var value = 0.toLong()
        for (byte in str.substring(6).chunked(5)) {
            value = (value * 16) + byte.substring(1).toInt(radix = 2)
            contentLen += 5
            if (byte[0] == '0') break
        }
        packet.value = value
    } else {
        val cond: (Int, Int, Int) -> Boolean
        if (str[6] == '0') {
            cond = { _, read, bound -> read < bound }
            headerLen += 16
        } else {
            cond = { i, _, bound -> i < bound }
            headerLen += 12
        }

        val bound = str.substring(7, headerLen).toInt(radix = 2)
        var i = 0
        while (cond(i, contentLen, bound)) {
            packet.innerPackets.add(parse(str.substring(headerLen + contentLen)))
            contentLen += packet.innerPackets.last().len
            i += 1
        }
    }

    packet.len = headerLen + contentLen
    return packet
}

fun String.toBin(): String = this.map {
    Integer.toBinaryString(it.toString().toInt(radix = 16)).padStart(4, '0')
}.joinToString("")

fun main() {
    val input = File("input").readText().trim().toBin()
    val packet = parse(input)

    println("First: ${packet.versionSum()}")
    println("Second: ${packet.eval()}")
}
