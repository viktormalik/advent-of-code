import java.io.File

fun segmentsByLen(len: Int): Set<Char> = when (len) {
    2 -> setOf('c', 'f')
    3 -> setOf('a', 'c', 'f')
    4 -> setOf('b', 'c', 'd', 'f')
    else -> ('a'..'g').toSet()
}

fun segmentToDigit(segment: Set<Char>): Char? = when (segment) {
    setOf('a', 'b', 'c', 'e', 'f', 'g') -> '0'
    setOf('c', 'f') -> '1'
    setOf('a', 'c', 'd', 'e', 'g') -> '2'
    setOf('a', 'c', 'd', 'f', 'g') -> '3'
    setOf('b', 'c', 'd', 'f') -> '4'
    setOf('a', 'b', 'd', 'f', 'g') -> '5'
    setOf('a', 'b', 'd', 'e', 'f', 'g') -> '6'
    setOf('a', 'c', 'f') -> '7'
    setOf('a', 'b', 'c', 'd', 'e', 'f', 'g') -> '8'
    setOf('a', 'b', 'c', 'd', 'f', 'g') -> '9'
    else -> null
}

fun MutableMap<Char, Set<Char>>.dropGroups(groups: Iterable<Set<Char>>) {
    for (group in groups) {
        for (c in filter { (_, map) -> map != group }.keys)
            merge(c, group, Set<Char>::minus)
    }
}

fun getOptions(digit: String, mapping: Map<Char, Set<Char>>): Set<Set<Char>> {
    if (digit.length == 1)
        return mapping.get(digit[0])!!.map { setOf(it) }.toSet()

    return getOptions(digit.substring(1), mapping)
        .flatMap { mapping.get(digit[0])!!.map { c -> setOf(c) + it } }
        .filter { it.size == digit.length }
        .toSet()
}

fun decode(entry: String): Int {
    val patterns = entry.split(" | ")[0]
    val mapping = ('a'..'g').map { it to ('a'..'g').toSet() }.toMap().toMutableMap()

    var size = mapping.map { (_, m) -> m.size }.sum()
    var prevSize = 0

    while (size != prevSize) {
        prevSize = size
        for (pat in patterns.split(" ")) {
            for (i in 0 until pat.length) {
                mapping.merge(pat[i], segmentsByLen(pat.length), Set<Char>::intersect)
            }
        }

        mapping.dropGroups(mapping.filter { (_, m) -> m.size == 1 }.values)
        mapping.dropGroups(
            mapping.filter { (c1, m1) ->
                m1.size == 2 && mapping.any { (c2, m2) -> c1 != c2 && m1 == m2 }
            }.values
        )

        size = mapping.map { (_, m) -> m.size }.sum()
    }

    return entry
        .split(" | ")[1]
        .split(" ")
        .map { getOptions(it, mapping).firstNotNullOf { segmentToDigit(it) } }
        .joinToString("")
        .toInt()
}

fun main() {
    val entries = File("input").readLines()

    val first = entries.map { e ->
        e.split(" | ")[1]
            .split(" ")
            .filter { d -> d.length == 2 || d.length == 3 || d.length == 4 || d.length == 7 }
            .count()
    }.sum()
    println("First: $first")

    val second = entries.map { decode(it) }.sum()
    println("Second: $second")
}
