import pathutils.Pos
import pathutils.fourNeighs
import pathutils.reachable
import java.io.File

fun neighs(pos: Pos, map: Array<IntArray>): List<Pos> =
    fourNeighs(pos, 0, map.size - 1, 0, map[0].size - 1)

fun basinNeighs(pos: Pos, map: Array<IntArray>): List<Pos> =
    neighs(pos, map).filter { map[it.x][it.y] != 9 }

fun main() {
    val map = File("input").readLines().map {
        it.map(Char::digitToInt).toIntArray()
    }.toTypedArray()
    val points = (0 until map.size).flatMap { x ->
        (0 until map[x].size).map { y -> Pos(x, y) }
    }

    val first = points.filter { p ->
        neighs(p, map).all { n -> map[n.x][n.y] > map[p.x][p.y] }
    }.map { map[it.x][it.y] + 1 }.sum()
    println("First: $first")

    val basins = mutableSetOf<Set<Pos>>()
    while (true) {
        val start = points.find { pt ->
            map[pt.x][pt.y] != 9 && basins.none { it.contains(pt) }
        } ?: break
        basins.add(reachable(start, map, ::basinNeighs))
    }
    val second = basins.sortedBy { -it.size }.take(3).fold(1) { res, basin -> res * basin.size }
    println("Second: $second")
}
