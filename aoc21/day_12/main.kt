import java.io.File

fun String.small(): Boolean = all { it.isLowerCase() }
fun List<String>.hasDuplicate() = size != distinct().size

fun countPaths(
    neighs: MutableMap<String, Set<String>>,
    mayVisit: (node: String, path: List<String>) -> Boolean,
): Int {
    val openPaths = mutableListOf(listOf("start"))
    var result = 0
    while (!openPaths.isEmpty()) {
        val p = openPaths.first()
        openPaths.removeFirst()

        for (n in neighs.get(p.last())!!) {
            if (n == "end") {
                result += 1
                continue
            }
            if (n == "start") continue
            if (mayVisit(n, p))
                openPaths.add(p + listOf(n))
        }
    }
    return result
}

fun main() {
    val neighs = mutableMapOf<String, Set<String>>()
    for (line in File("input").readLines()) {
        line.split("-").let {
            neighs.merge(it[0], setOf(it[1]), Set<String>::union)
            neighs.merge(it[1], setOf(it[0]), Set<String>::union)
        }
    }

    val first = countPaths(neighs) { n, p -> !(n.small() && p.contains(n)) }
    println("First: $first")

    val second = countPaths(neighs) { n, p ->
        !n.small() || p.count { it == n } == 0 ||
            (p.count { it == n } == 1 && !p.filter { it.small() }.hasDuplicate())
    }
    println("Second: $second")
}
