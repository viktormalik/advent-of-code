import java.io.File

data class Pos(val x: Int, val y: Int)
data class Path(val ps: List<Pos>, val risk: Int)

fun neighs(p: Pos, cave: Array<IntArray>): List<Pos> =
    listOf(Pos(p.x + 1, p.y), Pos(p.x - 1, p.y), Pos(p.x, p.y + 1), Pos(p.x, p.y - 1))
        .filter { it.x >= 0 && it.x < cave.size && it.y >= 0 && it.y < cave[0].size }

fun shortest(cave: Array<IntArray>): Int {
    var todo = mutableListOf<Path>(Path(listOf(Pos(0, 0)), 0))
    val seen = mutableSetOf<Pos>()

    while (!todo.isEmpty()) {
        val path = todo.first()
        todo.removeFirst()

        for (n in neighs(path.ps.last(), cave)) {
            if (!seen.contains(n)) {
                val newPath = Path(path.ps + listOf(n), path.risk + cave[n.x][n.y])
                todo.add(newPath)
                todo = todo.sortedBy { it.risk }.toMutableList()
                seen.add(n)
                if (n == Pos(cave.size - 1, cave[0].size - 1))
                    return newPath.risk
            }
        }
    }
    return 0
}

fun main() {
    val cave = File("input").readLines().map {
        it.map(Character::getNumericValue).toIntArray()
    }.toTypedArray()

    val first = shortest(cave)
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
    val second = shortest(largeCave)
    println("Second: $second")
}
