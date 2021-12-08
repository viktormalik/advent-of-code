import java.io.File

fun isTriangle(sides: List<Int>): Boolean {
    val (a, b, c) = sides
    return a + b > c && a + c > b && b + c > a
}

fun vertical(chunk: List<String>, idx: Int): List<Int> =
    chunk.map { it.trim().split("\\s+".toRegex()).get(idx).toInt() }

fun main() {
    val lines = File("input").readLines()

    val first = lines
        .filter { isTriangle(it.trim().split("\\s+".toRegex()).map { it.toInt() }) }
        .count()
    println("First: $first")

    val second = lines
        .chunked(3)
        .map { listOf(vertical(it, 0), vertical(it, 1), vertical(it, 2)) }
        .flatten()
        .filter { isTriangle(it) }
        .count()
    println("Second: $second")
}
