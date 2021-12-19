import java.io.File

data class Num(var num: Int, var depth: Int)
typealias Snailfish = MutableList<Num>

fun parseNum(str: String): Snailfish {
    val result = mutableListOf<Num>()
    var i = 0
    var depth = 0
    while (i < str.length) {
        when (str[i]) {
            '[' -> depth += 1
            ']' -> depth -= 1
            ',' -> {}
            else -> {
                val len = str.drop(i).indexOfFirst { !it.isDigit() }
                result.add(Num(str.substring(i, i + len).toInt(), depth))
                i += len - 1
            }
        }
        i += 1
    }
    return result
}

fun Snailfish.explode(): Boolean {
    val i = withIndex().find { (i, n) ->
        i < size - 1 && n.depth == this[i + 1].depth && n.depth > 4
    }?.index
    if (i == null)
        return false

    if (i > 0)
        this[i - 1].num += this[i].num
    if (i < size - 2)
        this[i + 2].num += this[i + 1].num

    removeAt(i)
    this[i] = Num(0, this[i].depth - 1)
    return true
}

fun Snailfish.split(): Boolean {
    val i = withIndex().find { (_, n) -> n.num >= 10 }?.index
    if (i == null)
        return false

    add(i + 1, Num(this[i].num / 2 + this[i].num % 2, this[i].depth + 1))
    this[i] = Num(this[i].num / 2, this[i].depth + 1)
    return true
}

fun Snailfish.reduce() {
    while (explode() || split()) {}
}

fun Snailfish.add(other: Snailfish): Snailfish {
    val sum = (this + other).map { Num(it.num, it.depth + 1) }.toMutableList()
    sum.reduce()
    return sum
}

fun Snailfish.magnitude(): Int {
    var muls = mutableListOf<Int>()
    var result = 0
    for (n in this) {
        while (n.depth > muls.size)
            muls.add(3)

        result += muls.fold(n.num) { prod, m -> prod * m }

        while (!muls.isEmpty() && muls.removeLast() == 2) {}
        muls.add(2)
    }
    return result
}

fun main() {
    val lines = File("input").readLines()
    val nums = lines.map { parseNum(it) }

    val first = nums.reduce { sum, num -> sum.add(num) }.magnitude()
    println("First: $first")

    val second = nums
        .flatMap { n1 -> nums.flatMap { n2 -> listOf(n1 to n2, n2 to n1) } }
        .map { (n1, n2) -> n1.add(n2).magnitude() }
        .maxOrNull()!!
    println("Second: $second")
}
