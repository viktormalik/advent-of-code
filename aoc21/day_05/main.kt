import java.io.File
import kotlin.math.abs

data class Pos(val x: Int, val y: Int)
data class Line(val from: Pos, val to: Pos)

fun parsePos(str: String) = str.split(",").let { Pos(it[0].toInt(), it[1].toInt()) }
fun parseLine(str: String): Line =
    str.split("->").let { Line(parsePos(it[0].trim()), parsePos(it[1].trim())) }

fun range(from: Int, to: Int) = if (from < to) (from..to) else (from downTo to)

fun main() {
    val lines = File("input").readLines().map { parseLine(it) }

    val points = mutableMapOf<Pos, Int>()
    for (line in lines) {
        if (line.from.x == line.to.x) {
            for (y in range(line.from.y, line.to.y))
                points.merge(Pos(line.from.x, y), 1, Int::plus)
        } else if (line.from.y == line.to.y) {
            for (x in range(line.from.x, line.to.x))
                points.merge(Pos(x, line.from.y), 1, Int::plus)
        }
    }

    val first = points.filter { (_, n) -> n > 1 }.count()
    println("First: $first")

    for (line in lines) {
        if (abs(line.from.x - line.to.x) == abs(line.from.y - line.to.y)) {
            for ((x, y) in range(line.from.x, line.to.x).zip(range(line.from.y, line.to.y)))
                points.merge(Pos(x, y), 1, Int::plus)
        }
    }

    val second = points.filter { (_, n) -> n > 1 }.count()
    println("Second: $second")
}
