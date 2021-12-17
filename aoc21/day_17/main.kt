import java.io.File
import kotlin.math.max

data class Pos(val x: Int, val y: Int)
data class Range(val min: Int, val max: Int) {
    fun contains(v: Int): Boolean = v >= min && v <= max
}

fun hit(x: Int, y: Int, xTarget: Range, yTarget: Range): Boolean {
    var pos = Pos(0, 0)
    var vel = Pos(x, y)
    while (true) {
        pos = Pos(pos.x + vel.x, pos.y + vel.y)
        vel = Pos(max(0, vel.x - 1), vel.y - 1)

        if (xTarget.contains(pos.x) && yTarget.contains(pos.y))
            return true
        else if (pos.y < yTarget.min) {
            return false
        }
    }
}

fun main() {
    val re = "target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)".toRegex()
    val match = re.find(File("input").readText())!!.groupValues
    val xTarget = Range(match.get(1).toInt(), match.get(2).toInt())
    val yTarget = Range(match.get(3).toInt(), match.get(4).toInt())

    val yMax = -yTarget.min - 1
    println("First: ${(yMax * yMax + yMax) / 2}")

    val second = (1..xTarget.max)
        .flatMap { x -> (yTarget.min..yMax).map { y -> Pos(x, y) } }
        .count { p -> hit(p.x, p.y, xTarget, yTarget) }
    println("Second: $second")
}
