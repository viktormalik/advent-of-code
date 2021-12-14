import java.io.File

class Polymer(var pairs: Map<String, Long>, val last: Char) {
    var chars = mutableMapOf<Char, Long>()

    constructor (str: String) : this(
        str.windowed(2).groupingBy { it }.eachCount().mapValues { it.value.toLong() }.toMap(),
        str.last(),
    )

    fun step(rules: Map<String, String>) {
        val newPairs = mutableMapOf<String, Long>()
        for ((pair, freq) in pairs) {
            newPairs.merge(pair[0] + rules.get(pair)!!, freq, Long::plus)
            newPairs.merge(rules.get(pair)!! + pair[1], freq, Long::plus)
        }
        pairs = newPairs
    }

    fun apply(rules: Map<String, String>, steps: Int) {
        repeat(steps) { step(rules) }
        chars = pairs
            .asIterable()
            .groupingBy { it.key[0] }
            .fold(0.toLong()) { sum, p -> sum + p.value }
            .toMap().toMutableMap()
        chars.merge(last, 1, Long::plus)
    }

    fun diff(): Long = chars.values.maxOrNull()!! - chars.values.minOrNull()!!
}

fun main() {
    val input = File("input").readLines()
    val rules = input.drop(2).map { it.split(" -> ").let { it[0] to it[1] } }.toMap()
    val polymer = Polymer(input[0])

    polymer.apply(rules, 10)
    println("First: ${polymer.diff()}")

    polymer.apply(rules, 30)
    println("Second: ${polymer.diff()}")
}
