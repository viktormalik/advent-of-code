import assembunny.Assembunny
import java.io.File

fun main() {
    val instructions = File("input").readLines()

    for (i in 0..1000) {
        val prog = Assembunny(instructions, mapOf('a' to i))
        prog.run(100)
        if (prog.stdout.toString() == "01".repeat(50)) {
            println("First: $i")
            break
        }
    }
}
