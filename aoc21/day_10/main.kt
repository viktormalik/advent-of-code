import java.io.File
import java.util.ArrayDeque

fun match(c: Char): Char = when (c) {
    '(' -> ')'
    '[' -> ']'
    '{' -> '}'
    '<' -> '>'
    else -> c
}

fun scoreErr(c: Char): Long = when (c) {
    ')' -> -3
    ']' -> -57
    '}' -> -1197
    '>' -> -25137
    else -> 0
}

fun scoreMissing(c: Char): Long = when (c) {
    ')' -> 1
    ']' -> 2
    '}' -> 3
    '>' -> 4
    else -> 0
}

fun analyse(line: String): Long {
    val stack = ArrayDeque<Char>()
    for (c in line) {
        when (c) {
            '(', '[', '<', '{' -> stack.push(c)
            else -> if (match(stack.pop()) != c) return scoreErr(c)
        }
    }
    return stack.fold(0) { res, c -> res * 5 + scoreMissing(match(c)) }
}

fun main() {
    val lines = File("input").readLines()

    val first = -lines.mapNotNull { analyse(it).takeIf { it < 0 } }.sum()
    println("First: $first")

    val scores = lines.mapNotNull { analyse(it).takeIf { it > 0 } }.sorted()
    println("Second: ${scores[scores.size / 2]}")
}
