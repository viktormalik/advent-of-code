import java.io.File

fun trap(row: CharArray, pos: Int): Boolean = pos >= 0 && pos < row.size && row[pos] == '^'

fun safeTiles(initRow: String, rowsCnt: Int): Int {
    val rows = mutableListOf(initRow.toCharArray())

    for (r in 1 until rowsCnt) {
        val lastRow = rows.last()
        rows.add(CharArray(lastRow.size) { '.' })
        val row = rows.last()

        for (i in 0 until row.size) {
            if (trap(lastRow, i - 1) && trap(lastRow, i) && !trap(lastRow, i + 1) ||
                !trap(lastRow, i - 1) && trap(lastRow, i) && trap(lastRow, i + 1) ||
                trap(lastRow, i - 1) && !trap(lastRow, i) && !trap(lastRow, i + 1) ||
                !trap(lastRow, i - 1) && !trap(lastRow, i) && trap(lastRow, i + 1)
            ) {
                row[i] = '^'
            }
        }
    }

    return rows.map { it.filter { it == '.' }.count() }.sum()
}

fun main() {
    val initRow = File("input").readText().trim()

    println("First: ${safeTiles(initRow, 40)}")
    println("Second: ${safeTiles(initRow, 400_000)}")
}
