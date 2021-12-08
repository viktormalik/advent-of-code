import java.io.File

fun getCode(
    instructions: List<String>,
    keypad: Array<Array<Char?>>,
    start_x: Int,
    start_y: Int
): String {
    val code = StringBuilder()
    var x = start_x
    var y = start_y

    for (inst in instructions) {
        for (dir in inst) {
            when (dir) {
                'U' -> if (x > 0 && keypad[x - 1][y] != null) x -= 1
                'D' -> if (x < keypad.size - 1 && keypad[x + 1][y] != null) x += 1
                'L' -> if (y > 0 && keypad[x][y - 1] != null) y -= 1
                'R' -> if (y < keypad[x].size - 1 && keypad[x][y + 1] != null) y += 1
            }
        }
        code.append(keypad[x][y])
    }

    return code.toString()
}

fun main() {
    val instructions = File("input").readLines()

    val keypad_first: Array<Array<Char?>> = arrayOf(
        arrayOf('1', '2', '3'),
        arrayOf('4', '5', '6'),
        arrayOf('7', '8', '9'),
    )
    val first = getCode(instructions, keypad_first, 1, 1)
    println("First: $first")

    val keypad_second: Array<Array<Char?>> = arrayOf(
        arrayOf(null, null, '1', null, null),
        arrayOf(null, '2', '3', '4', null),
        arrayOf('5', '6', '7', '8', '9'),
        arrayOf(null, 'A', 'B', 'C', null),
        arrayOf(null, null, 'D', null, null),
    )
    val second = getCode(instructions, keypad_second, 2, 0)
    println("Second: $second")
}
