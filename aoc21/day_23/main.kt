import pathutils.Pos
import pathutils.fourNeighs
import pathutils.reachable
import java.io.File
import kotlin.math.abs

enum class Room { INIT, HALL, DEST }

data class Pod(val type: Char, var pos: Pos, var room: Room = Room.INIT) {
    fun clone() = Pod(type, pos.clone(), room)

    fun energy(): Int = when (type) {
        'A' -> 1
        'B' -> 10
        'C' -> 100
        'D' -> 1000
        else -> 0
    }

    fun move(to: Pos) {
        pos = to
        room = when (room) {
            Room.INIT -> Room.HALL
            Room.HALL -> Room.DEST
            Room.DEST -> Room.DEST
        }
    }
}

fun neighs(pos: Pos, map: Array<CharArray>): List<Pos> =
    fourNeighs(pos).filter { map[it.x][it.y] == '.' }

class Burrow(val map: Array<CharArray>, var pods: List<Pod>, var energy: Int = 0) {
    fun clone() = Burrow(
        map.map { it.copyOf() }.toTypedArray(),
        pods.map { it.clone() },
        energy,
    )

    override fun toString(): String = map.map { it.joinToString("") }.joinToString("\n")

    fun destRoomCol(pod: Pod): Int = when (pod.type) {
        'A' -> 3
        'B' -> 5
        'C' -> 7
        'D' -> 9
        else -> 0
    }
    fun destRoom(pod: Pod): List<Pos> = (2..map.size - 2).map { Pos(it, destRoomCol(pod)) }

    fun moves(pod: Pod): List<Pos> = when (pod.room) {
        Room.INIT -> listOf(1, 2, 4, 6, 8, 10, 11).map { Pos(1, it) }
        Room.HALL -> destRoom(pod)
        Room.DEST -> listOf()
    }

    fun nextStates(): List<Burrow> {
        val result = mutableListOf<Burrow>()
        for (pod in pods) {
            val reachable = reachable(pod.pos, map, ::neighs)
            val destinations = moves(pod).intersect(reachable)
            for (dest in destinations) {
                if (pod.room == Room.HALL &&
                    destRoom(pod).any { p -> map[p.x][p.y] != '.' && map[p.x][p.y] != pod.type }
                ) {
                    continue
                }

                val newBurrow = clone()
                newBurrow.move(newBurrow.pods.find { it.pos == pod.pos }!!, dest)
                result.add(newBurrow)
            }
        }
        return result
    }

    fun move(pod: Pod, dest: Pos) {
        energy += (abs(dest.x - pod.pos.x) + abs(dest.y - pod.pos.y)) * pod.energy()
        map[pod.pos.x][pod.pos.y] = '.'
        pod.move(dest)
        map[pod.pos.x][pod.pos.y] = pod.type
    }

    fun done(): Boolean = pods.all { it.room == Room.DEST }
}

fun organize(map: Array<CharArray>): Int {
    val pods = map.withIndex().flatMap { (x, row) ->
        row.withIndex().filter { (_, c) -> c.isLetter() }.map { (y, c) -> Pod(c, Pos(x, y)) }
    }
    val burrow = Burrow(map, pods)

    var todo = mutableListOf(burrow)
    val seen = mutableMapOf(burrow.toString() to 0)
    var min: Int? = null
    while (!todo.isEmpty()) {
        val current = todo.first()
        todo.removeFirst()

        for (next in current.nextStates()) {
            val prev = seen.get(next.toString())
            if (prev != null && prev <= next.energy)
                continue
            if (min != null && next.energy >= min)
                continue
            if (next.done()) {
                if (min == null || min > next.energy)
                    min = next.energy
                continue
            }
            todo.add(next)
            seen.put(next.toString(), next.energy)
        }
    }
    return min!!
}

fun main() {
    val input = File("input").readLines()
    val map = input.map { it.toCharArray() }.toTypedArray()
    val firstMap = (input.take(3) + input.drop(5)).map { it.toCharArray() }.toTypedArray()

    val first = organize(firstMap)
    println("First: $first")
    val second = organize(map)
    println("Second: $second")
}
