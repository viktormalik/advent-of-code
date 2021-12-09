import java.io.File

data class Pt(val x: Int, val y: Int)

fun neighs(pt: Pt, map: Array<IntArray>): List<Pt> =
    listOf(
        Pt(pt.x + 1, pt.y),
        Pt(pt.x - 1, pt.y),
        Pt(pt.x, pt.y + 1),
        Pt(pt.x, pt.y - 1),
    ).filter { p -> p.x >= 0 && p.x < map.size && p.y >= 0 && p.y < map[0].size }

fun reachableFrom(start: Pt, map: Array<IntArray>): Set<Pt> {
    val todo = mutableListOf(start)
    val result = mutableSetOf(start)
    while (!todo.isEmpty()) {
        val pt = todo.first()
        todo.removeFirst()

        for (n in neighs(pt, map).filter { n -> map[n.x][n.y] != 9 && !result.contains(n) }) {
            todo.add(n)
            result.add(n)
        }
    }
    return result
}

fun main() {
    val map = File("input").readLines().map {
        it.map(Char::digitToInt).toIntArray()
    }.toTypedArray()
    val points = (0 until map.size).flatMap { x ->
        (0 until map[x].size).map { y -> Pt(x, y) }
    }

    val first = points.filter { p ->
        neighs(p, map).all { n -> map[n.x][n.y] > map[p.x][p.y] }
    }.map { map[it.x][it.y] + 1 }.sum()
    println("First: $first")

    val basins = mutableSetOf<Set<Pt>>()
    while (true) {
        val start = points.find { pt ->
            map[pt.x][pt.y] != 9 && basins.none { it.contains(pt) }
        } ?: break
        basins.add(reachableFrom(start, map))
    }
    val second = basins.sortedBy { -it.size }.take(3).fold(1) { res, basin -> res * basin.size }
    println("Second: $second")
}
