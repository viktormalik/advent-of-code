package pathutils

data class Pos(val x: Int, val y: Int)

fun <M> shortest(from: Pos, to: Pos, map: M, neighs: (Pos, M) -> List<Pos>): Int? {
    val todo = mutableListOf(from to 0)
    val seen = mutableSetOf<Pos>()

    while (!todo.isEmpty()) {
        val (pos, dst) = todo.first()
        todo.removeFirst()

        for (n in neighs(pos, map)) {
            if (!seen.contains(n)) {
                if (n == to)
                    return dst + 1
                todo.add(n to dst + 1)
                seen.add(n)
            }
        }
    }
    return null
}

fun <M> reachableIn(from: Pos, steps: Int, map: M, neighs: (Pos, M) -> List<Pos>): Int {
    val todo = mutableListOf(from to 0)
    val seen = mutableSetOf<Pos>()

    var cnt = 0
    while (!todo.isEmpty()) {
        val (pos, dst) = todo.first()
        todo.removeFirst()

        if (dst > steps)
            return cnt
        cnt += 1

        for (n in neighs(pos, map)) {
            if (!seen.contains(n)) {
                todo.add(n to dst + 1)
                seen.add(n)
            }
        }
    }
    return cnt
}
