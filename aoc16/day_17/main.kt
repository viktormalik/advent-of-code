import java.io.File
import java.security.MessageDigest
import kotlin.text.Charsets.UTF_8

data class Path(val path: String, val x: Int, val y: Int)

fun hash(str: String): String = MessageDigest
    .getInstance("MD5")
    .digest(str.toByteArray(UTF_8))
    .joinToString("") { byte -> "%02x".format(byte) }

fun isOpen(door: Char, x: Int, y: Int): Boolean =
    door >= 'b' && door <= 'f' && x >= 0 && x <= 3 && y >= 0 && y <= 3

fun next(path: Path, passcode: String): List<Path> {
    val doors = hash(passcode + path.path).substring(0, 4)
    val result = mutableListOf<Path>()

    if (isOpen(doors[0], path.x, path.y - 1))
        result.add(Path(path.path + "U", path.x, path.y - 1))
    if (isOpen(doors[1], path.x, path.y + 1))
        result.add(Path(path.path + "D", path.x, path.y + 1))
    if (isOpen(doors[2], path.x - 1, path.y))
        result.add(Path(path.path + "L", path.x - 1, path.y))
    if (isOpen(doors[3], path.x + 1, path.y))
        result.add(Path(path.path + "R", path.x + 1, path.y))

    return result
}

fun main() {
    val passcode = File("input").readText().trim()

    val paths = mutableListOf(Path("", 0, 0))
    val completePaths = mutableListOf<Path>()

    var first = true
    while (!paths.isEmpty()) {
        val path = paths.first()
        paths.removeFirst()

        if (path.x == 3 && path.y == 3) {
            if (first)
                println("First: ${path.path}")
            first = false
            completePaths.add(path)
            continue
        }

        paths.addAll(next(path, passcode))
    }

    println("Second: ${completePaths.last().path.length}")
}
