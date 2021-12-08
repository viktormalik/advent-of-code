import java.io.File

fun main() {
    val input = File("input").readText().trim().split(",").map { it.toInt() }
    val fish = (0..8)
        .map { n -> n to input.filter { it == n }.count().toLong() }
        .toMap()
        .toMutableMap()

    for (d in 0..255) {
        if (d == 80) println("First: ${fish.values.sum()}")
        val new = fish.get(0)!!
        for (i in 0..7)
            fish.put(i, fish.get(i + 1)!!)
        fish.put(8, new)
        fish.merge(6, new, Long::plus)
    }

    println("Second: ${fish.values.sum()}")
}
