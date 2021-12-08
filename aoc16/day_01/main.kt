import java.io.File
import kotlin.math.abs

data class Pos(var x: Int, var y: Int)

enum class Dir {
    N, S, E, W;
}

fun turn(current: Dir, turn: Char): Dir = when (current) {
    Dir.N -> if (turn == 'L') Dir.W else Dir.E
    Dir.S -> if (turn == 'L') Dir.E else Dir.W
    Dir.E -> if (turn == 'L') Dir.N else Dir.S
    Dir.W -> if (turn == 'L') Dir.S else Dir.N
}

fun main() {
    val instructions = File("input").readLines()[0].trim().split(", ")

    var dir = Dir.N
    var pos = Pos(0, 0)

    var seen = mutableSetOf<Pos>()
    var repeated: Pos? = null

    for (inst in instructions) {
        dir = turn(dir, inst[0])
        val steps = inst.substring(1).toInt()

        for (i in 0 until steps) {
            when (dir) {
                Dir.N -> pos.y += 1
                Dir.S -> pos.y -= 1
                Dir.E -> pos.x += 1
                Dir.W -> pos.x -= 1
            }

            if (seen.contains(pos) && repeated == null)
                repeated = pos.copy()

            seen.add(pos.copy())
        }
    }

    val first = abs(pos.x) + abs(pos.y)
    println("First: $first")

    if (repeated != null) {
        val second = abs(repeated.x) + abs(repeated.y)
        println("Second: $second")
    }
}
