import java.io.File

fun main() {
    var map = File("input").readLines().map { it.toCharArray() }.toTypedArray()

    var steps = 0
    var moved = true
    while (moved) {
        steps += 1
        moved = false

        var newMap = map.map { it.copyOf() }.toTypedArray()
        for (r in 0 until map.size) {
            for (c in 0 until map[r].size) {
                val nextC = (c + 1) % map[r].size
                if (map[r][c] == '>' && map[r][nextC] == '.') {
                    newMap[r][c] = '.'
                    newMap[r][nextC] = '>'
                    moved = true
                }
            }
        }
        map = newMap

        newMap = map.map { it.copyOf() }.toTypedArray()
        for (r in 0 until map.size) {
            for (c in 0 until map[r].size) {
                val nextR = (r + 1) % map.size
                if (map[r][c] == 'v' && map[nextR][c] == '.') {
                    newMap[r][c] = '.'
                    newMap[nextR][c] = 'v'
                    moved = true
                }
            }
        }
        map = newMap
    }

    println("First: $steps")
}
