import java.io.File

data class Pos(val x: Int, val y: Int)
data class Fold(val axe: Char, val n: Int)

fun fold(dots: Set<Pos>, f: Fold): Set<Pos> =
    if (f.axe == 'x') {
        dots.filter { it.x < f.n } +
            dots.filter { it.x > f.n }.map { Pos(f.n - (it.x - f.n), it.y) }
    } else {
        dots.filter { it.y < f.n } +
            dots.filter { it.y > f.n }.map { Pos(it.x, f.n - (it.y - f.n)) }
    }.toSet()

fun main() {
    val lines = File("input").readLines()

    var dots = lines
        .takeWhile { !it.isEmpty() }
        .map { it.split(",").map(String::toInt).let { Pos(it[0], it[1]) } }
        .toSet()
    var folds = lines
        .dropWhile { !it.isEmpty() }
        .drop(1)
        .map { Fold(it[11], it.split("=")[1].toInt()) }

    dots = fold(dots, folds[0])
    println("First: ${dots.size}")

    dots = folds.drop(1).fold(dots) { d, f -> fold(d, f) }
    println("Second:")
    for (y in 0..dots.maxByOrNull { it.y }!!.y) {
        for (x in 0..dots.maxByOrNull { it.x }!!.x) {
            if (dots.contains(Pos(x, y))) print('#')
            else print('.')
        }
        println()
    }
}
