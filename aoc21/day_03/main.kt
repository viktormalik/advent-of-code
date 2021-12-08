import java.io.File

fun List<String>.mcb(i: Int) =
    if (this.filter { it[i] == '1' }.count() >= this.size / 2) '1' else '0'

fun main() {
    val nums = File("input").readLines()

    val gamma = nums[0].withIndex().map { (i, _) -> nums.mcb(i) }.joinToString("")
    val epsilon = gamma.map { if (it == '1') '0' else '1' }.joinToString("")

    val first = gamma.toInt(radix = 2) * epsilon.toInt(radix = 2)
    println("First: $first")

    val o2 = (0..nums[0].length - 1).fold(nums) { n, i ->
        n.filter { it[i] == n.mcb(i) }
    }[0]
    val co2 = (0..nums[0].length - 1).fold(nums) { n, i ->
        n.filter { n.size == 1 || it[i] != n.mcb(i) }
    }[0]

    val second = o2.toInt(radix = 2) * co2.toInt(radix = 2)
    println("Second: $second")
}
