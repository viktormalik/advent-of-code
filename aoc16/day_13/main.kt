import pathutils.Pos
import pathutils.reachableIn
import pathutils.shortest
import java.io.File

fun isOpen(pos: Pos, favnum: Int): Boolean =
    (pos.x * pos.x + 3 * pos.x + 2 * pos.x * pos.y + pos.y + pos.y * pos.y + favnum)
        .countOneBits() % 2 == 0

fun neighs(pos: Pos, favnum: Int): List<Pos> =
    listOf(Pos(-1, 0), Pos(1, 0), Pos(0, -1), Pos(0, 1))
        .map { shift -> Pos(pos.x + shift.x, pos.y + shift.y) }
        .filter { p -> p.x >= 0 && p.y >= 0 && isOpen(p, favnum) }

fun main() {
    val favnum = File("input").readText().trim().toInt()

    val first = shortest(Pos(1, 1), Pos(31, 39), favnum, ::neighs)
    println("First: $first")

    val second = reachableIn(Pos(1, 1), 50, favnum, ::neighs) - 1
    println("Second: $second")
}
