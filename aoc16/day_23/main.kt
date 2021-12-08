import assembunny.Assembunny
import java.io.File

fun main() {
    val insts = File("input").readLines()

    val first = Assembunny(insts, mapOf('a' to 7))
    println("First: ${first.run()}")

    val second = Assembunny(insts, mapOf('a' to 12))
    println("Second: ${second.run()}")
}
