import java.io.File

data class Disc(val size: Int, val start: Int)

fun parseDisc(line: String): Disc =
    line.split(" ").let { Disc(it[3].toInt(), it[11].dropLast(1).toInt()) }

fun main() {
    val discs = File("input").readLines().map { parseDisc(it) }.toMutableList()

    var t = 0
    while (true) {
        if (discs.withIndex().all { (i, d) -> (d.start + t + i + 1) % d.size == 0 }) {
            println("First: $t")
            break
        }
        t += 1
    }

    discs.add(Disc(11, 0))
    t = 0
    while (true) {
        if (discs.withIndex().all { (i, d) -> (d.start + t + i + 1) % d.size == 0 }) {
            println("Second: $t")
            break
        }
        t += 1
    }
}
