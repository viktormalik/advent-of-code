import assembunny.Assembunny
import java.io.File

fun main() {
    val instructions = File("input").readLines()

    val first = Assembunny(instructions)
    println("First: ${first.run()}")

    val second = Assembunny(instructions, mapOf('c' to 1))
    println("Second: ${second.run()}")
}
