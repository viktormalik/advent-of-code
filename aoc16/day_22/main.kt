import pathutils.Pos
import pathutils.shortest
import java.io.File

data class Node(val size: Int, val used: Int, var target: Boolean)

fun parsePos(line: String): Pos = with(line.split(" ")[0].split("-")) {
    Pos(this[1].drop(1).toInt(), this[2].drop(1).toInt())
}

fun parseNode(line: String): Node = with(line.split("\\s+".toRegex())) {
    Node(this[1].dropLast(1).toInt(), this[2].dropLast(1).toInt(), false)
}

fun fits(n1: Node, n2: Node): Boolean =
    n1 !== n2 && n1.used > 0 && n1.used <= n2.size - n2.used

fun interchangeable(n1: Node, n2: Node): Boolean =
    n1.size > n2.used && n2.size > n1.used && !n1.target && !n2.target

fun isNeighbor(pos: Pos, neigh: Pos, nodes: Map<Pos, Node>): Boolean =
    nodes[neigh] != null && interchangeable(nodes[neigh]!!, nodes[pos]!!)

fun neighs(pos: Pos, nodes: Map<Pos, Node>): List<Pos> =
    listOf(
        Pos(pos.x + 1, pos.y), Pos(pos.x - 1, pos.y),
        Pos(pos.x, pos.y + 1), Pos(pos.x, pos.y - 1),
    ).filter { isNeighbor(pos, it, nodes) }

fun main() {
    val nodes = File("input").readLines().drop(2).map { parsePos(it) to parseNode(it) }.toMap()

    val first = nodes
        .flatMap { (_, n1) -> nodes.map { (_, n2) -> n1 to n2 } }
        .filter { (n1, n2) -> fits(n1, n2) }
        .count()
    println("First: $first")

    val mainPath = nodes.keys.filter { it.y == 0 }.sortedBy { -it.x }.toMutableList()
    var target = mainPath.first()
    mainPath.removeFirst()
    nodes[target]!!.target = true

    var empty = nodes.filterValues { it.used == 0 }.keys.first()
    var totalLen = 0

    while (!mainPath.isEmpty()) {
        val nextTarget = mainPath.first()
        mainPath.removeFirst()
        totalLen += shortest(empty, nextTarget, nodes, ::neighs)!! + 1
        nodes[target]!!.target = false
        nodes[nextTarget]!!.target = true
        empty = target
        target = nextTarget
    }
    println("Second: $totalLen")
}
