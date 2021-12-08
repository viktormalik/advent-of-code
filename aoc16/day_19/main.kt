import java.io.File

fun main() {
    val n = File("input").readText().trim().toInt()

    val bin = n.toString(radix = 2)
    val first = (bin.drop(1) + bin[0]).toInt(radix = 2)
    println("First: $first")

    var start = n / 2
    var shift = if (n % 2 == 0) 2 else 1
    var elves = List(n, { it + 1 })

    while (elves.size > 1) {
        val newShift = (3 - ((elves.size - (start + shift)) % 3)) % 3
        elves = elves.slice(0..start - 1) + elves.slice(start + shift..elves.size - 1 step 3)
        start = 0
        shift = newShift
    }
    println("Second: ${elves[0]}")
}
