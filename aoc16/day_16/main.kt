import java.io.File

fun checksum(input: String, len: Int): String {
    var data = input
    while (data.length < len) {
        val a = data
        val b = data.reversed().map { if (it == '0') '1' else '0' }.joinToString("")
        data = a + "0" + b
    }
    data = data.substring(0, len)

    var checksum = data
    while (checksum.length % 2 == 0) {
        checksum = checksum.chunked(2).map { if (it[0] == it[1]) '1' else '0' }.joinToString("")
    }
    return checksum
}

fun main() {
    val input = File("input").readText().trim()

    println("First: ${checksum(input, 272)}")
    println("Second: ${checksum(input, 35651584)}")
}
