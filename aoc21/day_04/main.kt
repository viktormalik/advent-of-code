import java.io.File

data class Num(val n: Int, var marked: Boolean)

class Board(val nums: Array<Array<Num>>) {
    var won = false

    fun fullRow(i: Int): Boolean = nums[i].all { it.marked }
    fun fullCol(i: Int): Boolean = nums.all { it[i].marked }
    fun mark(n: Int) {
        nums.forEach { it.forEach { if (it.n == n) it.marked = true } }
        if ((0..4).any { fullRow(it) || fullCol(it) }) won = true
    }
    fun score(): Int = nums.map { it.filter { !it.marked }.map { it.n }.sum() }.sum()
}

fun main() {
    val input = File("input").readText().trim().split("\n\n")

    val nums = input[0].trim().split(",").map { it.toInt() }
    val boards = input.drop(1).map {
        Board(
            it.trim().split("\n").map { line ->
                line.trim().split("\\s+".toRegex()).map { n ->
                    Num(n.toInt(), false)
                }.toTypedArray()
            }.toTypedArray()
        )
    }

    var first: Int? = null
    for (n in nums) {
        for (b in boards) {
            if (!b.won) b.mark(n)
            if (b.won && first == null) first = b.score() * n
            if (boards.all { it.won }) {
                println("First: $first")
                println("Second: ${b.score() * n}")
                return
            }
        }
    }
}
