import java.io.File
import kotlin.math.min

fun getVal(v: String, regs: Map<Char, Long>): Long =
    if (v[0].isLetter()) regs.get(v[0])!! else v.toLong()

fun execSegment(input: Char, initZ: Long, code: List<String>): Long {
    val regs = ('x'..'z').map { it to 0.toLong() }.toMap().toMutableMap()
    regs.put('z', initZ)
    for (line in code) {
        val inst = line.split(" ")
        when (inst[0]) {
            "inp" -> regs.put(inst[1][0], input.toString().toLong())
            "add" -> regs.put(inst[1][0], getVal(inst[1], regs) + getVal(inst[2], regs))
            "mul" -> regs.put(inst[1][0], getVal(inst[1], regs) * getVal(inst[2], regs))
            "div" -> regs.put(inst[1][0], getVal(inst[1], regs) / getVal(inst[2], regs))
            "mod" -> regs.put(inst[1][0], getVal(inst[1], regs) % getVal(inst[2], regs))
            "eql" -> regs.put(
                inst[1][0],
                if (getVal(inst[1], regs) == getVal(inst[2], regs)) 1 else 0
            )
        }
    }
    return regs.get('z')!!
}

fun main() {
    val input = File("input").readLines().asSequence()
    val segmentSize = input.drop(1).takeWhile { !it.startsWith("inp") }.count() + 1

    var inputValues = mutableMapOf(0.toLong() to mutableListOf(""))
    for (segment in input.chunked(segmentSize).toList().asReversed()) {
        val newInputValues = mutableMapOf<Long, MutableList<String>>()
        val maxZ = min(inputValues.keys.maxOrNull()!! * 26 + 26, 1000000)

        for (w in '1'..'9') {
            for (z in 0..maxZ) {
                val segmentRes = execSegment(w, z.toLong(), segment)

                val numbers = inputValues.get(segmentRes)
                if (numbers != null) {
                    for (number in numbers) {
                        if (!newInputValues.containsKey(z.toLong()))
                            newInputValues.put(z.toLong(), mutableListOf<String>())
                        newInputValues.get(z.toLong())!!.add(w + number)
                    }
                }
            }
        }

        inputValues = newInputValues
    }

    println("First: ${inputValues.get(0)?.maxOrNull()!!}")
    println("Second: ${inputValues.get(0)?.minOrNull()!!}")
}
