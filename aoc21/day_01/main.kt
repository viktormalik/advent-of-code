import java.io.File

fun List<Int>.increasing(): Int = this.windowed(2).filter { it[1] > it[0] }.count()

fun main() {
    val measures = File("input").readLines().map { it.toInt() }

    val first = measures.increasing()
    println("First: $first")

    val second = measures.windowed(3).map { it.sum() }.increasing()
    println("Second: $second")
}
