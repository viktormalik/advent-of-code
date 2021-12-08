import java.io.File

fun String.name() = this.split(" ")[0]
fun String.x() = this.split(" ")[1].toInt()

data class Pos(val horiz: Int, val depth: Int, val aim: Int) {
    fun nextFirst(cmd: String, x: Int) = when (cmd) {
        "forward" -> Pos(horiz + x, depth, aim)
        "down" -> Pos(horiz, depth + x, aim)
        "up" -> Pos(horiz, depth - x, aim)
        else -> this
    }

    fun nextSecond(cmd: String, x: Int) = when (cmd) {
        "forward" -> Pos(horiz + x, depth + aim * x, aim)
        "down" -> Pos(horiz, depth, aim + x)
        "up" -> Pos(horiz, depth, aim - x)
        else -> this
    }
}

fun main() {
    val insts = File("input").readLines()

    val first = insts.fold(Pos(0, 0, 0)) { pos, cmd -> pos.nextFirst(cmd.name(), cmd.x()) }
    println("First: ${first.horiz * first.depth}")

    val second = insts.fold(Pos(0, 0, 0)) { pos, cmd -> pos.nextSecond(cmd.name(), cmd.x()) }
    println("Second: ${second.horiz * second.depth}")
}
