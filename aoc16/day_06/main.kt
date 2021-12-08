import java.io.File

fun main() {
    val lines = File("input").readLines()

    val freqs = Array(lines[0].length) { mutableMapOf<Char, Int>() }

    for (line in lines) {
        for (i in 0..line.length - 1)
            freqs[i].merge(line[i], 1, Int::plus)
    }

    val first = freqs
        .map { f -> f.toList().sortedBy { (_, v) -> -v }[0].first }
        .joinToString("")
    println("First: $first")

    val second = freqs
        .map { f -> f.toList().sortedBy { (_, v) -> v }[0].first }
        .joinToString("")
    println("Second: $second")
}
