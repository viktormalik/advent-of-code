import java.io.File

data class Pixel(val r: Int, val c: Int)

fun area(rows: IntRange, cols: IntRange): List<Pixel> =
    rows.flatMap { r -> cols.map { c -> Pixel(r, c) } }

class Picture(var pixels: Set<Pixel>) {
    var min = Pixel(-1, -1)
    var max = Pixel(
        pixels.map { it.r }.maxOrNull()!! + 1,
        pixels.map { it.c }.maxOrNull()!! + 1,
    )
    var outerOnes = false

    fun outer(p: Pixel): Boolean = p.r <= min.r || p.r >= max.r || p.c <= min.c || p.c >= max.c

    fun pixelId(p: Pixel): Int =
        area(p.r - 1..p.r + 1, p.c - 1..p.c + 1)
            .map { if (pixels.contains(it) || (outer(it) && outerOnes)) '1' else '0' }
            .joinToString("")
            .toInt(radix = 2)

    fun enhance(algo: CharArray) {
        pixels = area(min.r..max.r, min.c..max.c)
            .filter { p -> algo[pixelId(p)] == '#' }
            .toSet()
        min = Pixel(min.r - 1, min.c - 1)
        max = Pixel(max.r + 1, max.c + 1)
        outerOnes = !outerOnes
    }
}

fun main() {
    val input = File("input").readLines()
    val algo = input[0].toCharArray()

    val picture = Picture(
        input.drop(2).withIndex().flatMap { (r, row) ->
            row.withIndex().filter { (_, x) -> x == '#' }.map { (c, _) -> Pixel(r, c) }
        }.toSet()
    )

    repeat(2) { picture.enhance(algo) }
    println("First: ${picture.pixels.size}")

    repeat(48) { picture.enhance(algo) }
    println("Second: ${picture.pixels.size}")
}
