import java.io.File
import kotlin.math.max
import kotlin.math.min

data class Range(val min: Int, val max: Int) {
    fun hasIntersection(range: Range): Boolean = range.min <= max && range.max >= min

    fun intersection(range: Range): Range = Range(max(min, range.min), min(max, range.max))

    fun minus(range: Range): List<Range> {
        if (!hasIntersection(range)) return listOf(this)
        if (range.min <= min && range.max >= max) return listOf()
        if (range.min < min) return listOf(Range(range.max + 1, max))
        if (range.max > max) return listOf(Range(min, range.min - 1))
        return listOf(Range(min, range.min - 1), Range(range.max + 1, max))
    }

    fun size(): Long = (max - min + 1).toLong()
}

class Cube(val x: Range, val y: Range, val z: Range) {
    fun hasIntersection(cube: Cube): Boolean =
        x.hasIntersection(cube.x) && y.hasIntersection(cube.y) && z.hasIntersection(cube.z)

    fun intersection(cube: Cube): Cube? =
        if (hasIntersection(cube))
            Cube(x.intersection(cube.x), y.intersection(cube.y), z.intersection(cube.z))
        else null

    fun minus(cube: Cube): List<Cube> =
        x.minus(cube.x).map { Cube(it, y, z) } +
            y.minus(cube.y).map { Cube(x.intersection(cube.x), it, z) } +
            z.minus(cube.z).map { Cube(x.intersection(cube.x), y.intersection(cube.y), it) }

    fun size(): Long = x.size() * y.size() * z.size()
}

fun main() {
    var cubes = listOf<Cube>()

    val re = "x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)".toRegex()
    for (line in File("input").readLines()) {
        val match = re.find(line)!!.groupValues
        val cube = Cube(
            Range(match.get(1).toInt(), match.get(2).toInt()),
            Range(match.get(3).toInt(), match.get(4).toInt()),
            Range(match.get(5).toInt(), match.get(6).toInt()),
        )

        if (line.startsWith("on")) {
            cubes += cubes.fold(listOf(cube)) { newCubes, c ->
                newCubes.flatMap {
                    if (it.hasIntersection(c)) it.minus(c) else listOf(it)
                }
            }
        } else {
            cubes = cubes.flatMap {
                if (it.hasIntersection(cube)) it.minus(cube) else listOf(it)
            }
        }
    }

    val initArea = Cube(Range(-50, 50), Range(-50, 50), Range(-50, 50))
    val first = cubes.map { it.intersection(initArea)?.size() ?: 0 }.sum()
    println("First: $first")
    val second = cubes.map { it.size() }.sum()
    println("Second: $second")
}
