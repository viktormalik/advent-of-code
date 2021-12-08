import java.io.File
import kotlin.math.min

data class Range(val min: Long, val max: Long)

fun main() {
    val denylist = File("input")
        .readLines()
        .map { Range(it.split("-")[0].toLong(), it.split("-")[1].toLong()) }

    var validRanges = setOf(Range(0, 4294967295))
    for (deny in denylist) {
        validRanges = validRanges
            .flatMap { r ->
                if (deny.min <= r.min && deny.max >= r.max)
                    listOf(null)
                else if (deny.min <= r.min && deny.max > r.min)
                    listOf(Range(deny.max + 1, r.max))
                else if (deny.max >= r.max && deny.min < r.max)
                    listOf(Range(r.min, deny.min - 1))
                else if (deny.min > r.min && deny.max < r.max)
                    listOf(Range(r.min, deny.min - 1), Range(deny.max + 1, r.max))
                else
                    listOf(r)
            }
            .filterNotNull()
            .toSet()
    }

    val first = validRanges.minByOrNull { it.min }!!.min
    println("First: $first")

    val second = validRanges.map { it.max - it.min + 1 }.sum()
    println("Second: $second")
}
