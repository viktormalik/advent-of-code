import pathutils.Pos
import pathutils.fourNeighs
import pathutils.reachable
import pathutils.shortest
import java.io.File

fun isOpen(pos: Pos, favnum: Int): Boolean =
    (pos.x * pos.x + 3 * pos.x + 2 * pos.x * pos.y + pos.y + pos.y * pos.y + favnum)
        .countOneBits() % 2 == 0

fun neighs(pos: Pos, favnum: Int): List<Pos> =
    fourNeighs(pos, xMin = 0, yMin = 0).filter { isOpen(it, favnum) }

fun main() {
    val favnum = File("input").readText().trim().toInt()

    val first = shortest(Pos(1, 1), Pos(31, 39), favnum, ::neighs, { _, _, _ -> 1 })!!.len()
    println("First: $first")

    val second = reachable(Pos(1, 1), favnum, ::neighs, 50).size
    println("Second: $second")
}
