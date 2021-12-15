package pathutils

data class Pos(val x: Int, val y: Int)
data class Path(val ps: List<Pos>, val dst: Int) {
    fun len(): Int = ps.size - 1
}

fun fourNeighs(
    pos: Pos,
    xMin: Int? = null,
    xMax: Int? = null,
    yMin: Int? = null,
    yMax: Int? = null
): List<Pos> =
    listOf(
        Pos(pos.x + 1, pos.y),
        Pos(pos.x - 1, pos.y),
        Pos(pos.x, pos.y + 1),
        Pos(pos.x, pos.y - 1)
    ).filter {
        (xMin == null || it.x >= xMin) &&
            (xMax == null || it.x <= xMax) &&
            (yMin == null || it.y >= yMin) &&
            (yMax == null || it.y <= yMax)
    }

fun <M> shortest(
    from: Pos,
    to: Pos,
    map: M,
    neighs: (Pos, M) -> List<Pos>,
    dst: ((Pos, Pos, M) -> Int)? = null,
): Path? {
    var todo = mutableListOf<Path>(Path(listOf(from), 0))
    val seen = mutableSetOf<Pos>()

    while (!todo.isEmpty()) {
        val path = todo.first()
        todo.removeFirst()

        for (n in neighs(path.ps.last(), map)) {
            if (!seen.contains(n)) {
                val newPath = Path(
                    path.ps + listOf(n),
                    path.dst + (dst?.invoke(path.ps.last(), n, map) ?: 1)
                )
                todo.add(newPath)
                if (dst != null)
                    todo = todo.sortedBy { it.dst }.toMutableList()
                seen.add(n)
                if (n == to)
                    return newPath
            }
        }
    }
    return null
}

fun <M> reachable(
    from: Pos,
    map: M,
    neighs: (Pos, M) -> List<Pos>,
    maxSteps: Int? = null
): Set<Pos> {
    val todo = mutableListOf(from to 0)
    val reachable = mutableSetOf<Pos>()

    while (!todo.isEmpty()) {
        val (pos, dst) = todo.first()
        todo.removeFirst()

        for (n in neighs(pos, map)) {
            if (!reachable.contains(n)) {
                if (maxSteps == null || dst < maxSteps) {
                    todo.add(n to dst + 1)
                    reachable.add(n)
                }
            }
        }
    }
    return reachable
}
