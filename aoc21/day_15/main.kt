import pathutils.Pos
import pathutils.fourNeighs
import pathutils.shortest
import java.io.File

fun neighs(pos: Pos, cave: Array<IntArray>): List<Pos> =
    fourNeighs(pos, 0, cave.size - 1, 0, cave[0].size - 1)

fun dstFun(@Suppress("UNUSED_PARAMETER") from: Pos, to: Pos, cave: Array<IntArray>): Int =
    cave[to.x][to.y]

fun main() {
    val cave = File("input").readLines().map {
        it.map(Character::getNumericValue).toIntArray()
    }.toTypedArray()

    val firstDest = Pos(cave.size - 1, cave[0].size - 1)
    val first = shortest(Pos(0, 0), firstDest, cave, ::neighs, ::dstFun)!!.dst
    println("First: $first")

    val largeCave = Array(cave.size * 5) { IntArray(cave[0].size * 5) }
    for (x in 0 until cave.size) {
        for (i in 0 until 5) {
            for (y in 0 until cave[x].size) {
                for (j in 0 until 5) {
                    largeCave[cave.size * i + x][cave[x].size * j + y] =
                        (cave[x][y] + i + j).let { if (it >= 10) (it % 10) + 1 else it }
                }
            }
        }
    }

    val secondDest = Pos(largeCave.size - 1, largeCave[0].size - 1)
    val second = shortest(Pos(0, 0), secondDest, largeCave, ::neighs, ::dstFun)!!.dst
    println("Second: $second")
}
