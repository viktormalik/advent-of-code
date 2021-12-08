import java.io.File

fun print_display(display: Array<Array<Boolean>>) =
    display.forEach { it.forEach { print(if (it) '#' else '.') }; println() }

fun main() {
    val instructions = File("input").readLines()
    val display = Array(6) { Array(50) { false } }

    for (inst in instructions) {
        if (inst.startsWith("rect")) {
            val A = inst.split(" ")[1].split("x")[0].toInt()
            val B = inst.split(" ")[1].split("x")[1].toInt()

            display
                .take(B)
                .forEach { row -> row.take(A).forEachIndexed { i, _ -> row[i] = true } }
        } else if (inst.startsWith("rotate row")) {
            val A = inst.split("y=")[1].split("by")[0].trim().toInt()
            val B = inst.split("y=")[1].split("by")[1].trim().toInt()

            display[A] = display[A].sliceArray(50 - B until 50) +
                display[A].sliceArray(0 until 50 - B)
        } else if (inst.startsWith("rotate column")) {
            val A = inst.split("x=")[1].split("by")[0].trim().toInt()
            val B = inst.split("x=")[1].split("by")[1].trim().toInt()

            val column = display.map { it[A] }.slice(6 - B until 6) +
                display.map { it[A] }.slice(0 until 6 - B)
            for (i in 0 until column.size) {
                display[i][A] = column[i]
            }
        }
    }

    val first = display.map { it.filter { it }.count() }.sum()
    println("First: $first")

    println("Second:")
    print_display(display)
}
