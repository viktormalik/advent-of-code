import pathutils.Pos
import pathutils.fourNeighs
import pathutils.shortest
import java.io.File

fun neighs(pos: Pos, map: List<CharArray>): List<Pos> =
    fourNeighs(pos).filter { p -> map[p.x][p.y] != '#' }

fun allDsts(pois: Map<Char, Pos>, map: List<CharArray>): Map<String, Int> {
    val dsts = mutableMapOf<String, Int>()
    for ((p1, pos1) in pois) {
        for ((p2, pos2) in pois) {
            if (p1 != p2) {
                val d = shortest(pos1, pos2, map, ::neighs)!!.len()
                dsts.put("" + p1 + p2, d)
                dsts.put("" + p2 + p1, d)
            }
        }
    }
    return dsts
}

fun permutations(path: String): List<String> {
    if (path.isEmpty())
        return listOf(path)

    val result = mutableListOf<String>()
    for ((i, p) in path.withIndex()) {
        for (perm in permutations(path.removeRange(i, i + 1)))
            result.add(p + perm)
    }
    return result
}

fun shortestPath(start: Char, end: String, through: String, dsts: Map<String, Int>): Int =
    permutations(through).map { perm ->
        (perm + end).fold(start to 0) { (curr, dst), next ->
            next to dst + dsts["" + curr + next]!!
        }.second
    }.minOrNull()!!

fun main() {
    val map = File("input").readLines().map { it.toCharArray() }

    val pois = map
        .withIndex()
        .flatMap { (x, row) ->
            row
                .withIndex()
                .filter { (_, c) -> c.isDigit() }
                .map { (y, poi) -> poi to Pos(x, y) }
        }.toMap()
        .toMutableMap()

    val dsts = allDsts(pois, map)
    pois.remove('0')

    val first = shortestPath('0', "", pois.keys.joinToString(""), dsts)
    println("First: $first")

    val second = shortestPath('0', "0", pois.keys.joinToString(""), dsts)
    println("Second: $second")
}
