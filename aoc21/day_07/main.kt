import java.io.File
import kotlin.math.abs

fun main() {
    val nums = File("input").readText().trim().split(",").map { it.toInt() }

    val first = (nums.minOrNull()!!..nums.maxOrNull()!!)
        .map { n -> nums.map { abs(it - n) }.sum() }
        .minOrNull()!!
    println("First: $first")

    val second = (nums.minOrNull()!!..nums.maxOrNull()!!)
        .map { n -> nums.map { abs(it - n).let { x -> (x * x + x) / 2 } }.sum() }
        .minOrNull()!!
    println("Second: $second")
}
