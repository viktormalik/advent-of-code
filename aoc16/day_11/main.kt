import java.io.File

data class State(val floors: Array<MutableSet<String>>, val elevator: Int) {
    var moves: Int = 0

    constructor () : this(Array(4) { mutableSetOf() }, 0)

    fun deepCopy(
        floors: Array<MutableSet<String>> =
            this.floors.map { objs -> objs.toMutableSet() }.toTypedArray(),
        elevator: Int = this.elevator,
    ) = State(floors, elevator)

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other?.javaClass != javaClass) return false

        other as State

        for (f in 0..floors.size - 1) {
            if (pairsAtFloor(f) != other.pairsAtFloor(f) ||
                chipsAtFloor(f) != other.chipsAtFloor(f) ||
                gensAtFloor(f) != other.gensAtFloor(f)
            ) {
                return false
            }
        }

        return elevator == other.elevator
    }

    override fun hashCode(): Int =
        elevator * 31 + floors.withIndex().fold(1) { h, (i, _) ->
            (((h * 31 + pairsAtFloor(i)) * 31) + chipsAtFloor(i)) * 31 + gensAtFloor(i)
        }

    fun chip(obj: String): Boolean = obj[1] == 'm'
    fun gen(obj: String): Boolean = obj[1] == 'g'

    fun pairsAtFloor(f: Int): Int =
        floors[f].filter { c -> chip(c) && floors[f].any { g -> gen(g) && c[0] == g[0] } }.count()

    fun chipsAtFloor(f: Int): Int =
        floors[f].filter { c -> chip(c) && floors[f].none { g -> gen(g) && c[0] == g[0] } }.count()

    fun gensAtFloor(f: Int): Int =
        floors[f].filter { g -> gen(g) && floors[f].none { c -> chip(c) && c[0] == g[0] } }.count()

    fun safe() =
        floors.none { f ->
            f.any { c ->
                chip(c) &&
                    f.any { g -> gen(g) && g[0] != c[0] } &&
                    f.none { g -> gen(g) && g[0] == c[0] }
            }
        }

    fun up(objs: List<String>): State? {
        if (elevator < 3) {
            val newState = deepCopy(elevator = this.elevator + 1)
            newState.floors[elevator].removeAll(objs)
            newState.floors[elevator + 1].addAll(objs)
            newState.moves = moves + 1
            return newState
        }
        return null
    }

    fun down(objs: List<String>): State? {
        if (elevator > 0) {
            val newState = deepCopy(elevator = this.elevator - 1)
            newState.floors[elevator].removeAll(objs)
            newState.floors[elevator - 1].addAll(objs)
            newState.moves = moves + 1
            return newState
        }
        return null
    }

    fun nextStates(): List<State> {
        val result = mutableListOf<State>()

        for (obj in floors[elevator]) {
            up(listOf(obj))?.let { result.add(it) }
            down(listOf(obj))?.let { result.add(it) }
        }

        for (
            objList in floors[elevator]
                .flatMap { o1 -> floors[elevator].map { o2 -> (o1 to o2) } }
                .filter { (o1, o2) -> o1 != o2 }
                .map { (o1, o2) -> listOf(o1, o2) }
        ) {
            up(objList)?.let { result.add(it) }
            down(objList)?.let { result.add(it) }
        }

        return result
    }

    fun isFinal(): Boolean =
        floors.take(3).all { it.none() }
}

fun computeSteps(initState: State): Int {
    val todo = mutableListOf(initState.deepCopy())
    val seen = mutableSetOf<State>()

    while (!todo.isEmpty()) {
        val current = todo.first()
        todo.removeFirst()

        if (current.isFinal()) {
            return current.moves
        }

        for (next in current.nextStates()) {
            if (next.safe() && seen.none { it == next }) {
                todo.add(next)
                seen.add(next)
            }
        }
    }

    return 0
}

fun main() {
    val lines = File("input").readLines()

    val initState = State()
    for (f in 0..lines.size - 2) {
        val line = lines[f].split(" ").drop(4).joinToString(" ")
        line
            .split(", ")
            .map {
                val words = it.split(" ")
                String(listOf(words[words.size - 2][0], words[words.size - 1][0]).toCharArray())
            }
            .forEach { initState.floors[f].add(it) }
    }

    println("First: ${computeSteps(initState)}")

    initState.floors[0].addAll(listOf("eg", "em", "dg", "dm"))
    println("Second: ${computeSteps(initState)}")
}
