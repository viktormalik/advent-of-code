import java.io.File
import kotlin.math.abs

enum class Axe {
    X, Y, Z;

    companion object {
        fun allRotations(): List<List<Axe>> = listOf(
            // facing
            listOf(), // x
            listOf(Axe.Y, Axe.Y), // -x
            listOf(Axe.Z), // y
            listOf(Axe.Z, Axe.Z, Axe.Z), // -y
            listOf(Axe.Y), // z
            listOf(Axe.Y, Axe.Y, Axe.Y), // -z
        ).flatMap { rot ->
            listOf(
                // try all rotations of that facing
                rot,
                rot + listOf(Axe.X),
                rot + listOf(Axe.X, Axe.X),
                rot + listOf(Axe.X, Axe.X, Axe.X)
            )
        }
    }
}

data class Pos(val x: Int, val y: Int, val z: Int) {
    fun shift(rel: Pos): Pos = Pos(x + rel.x, y + rel.y, z + rel.z)

    fun rotate(axe: Axe): Pos = when (axe) {
        Axe.X -> Pos(x, z, -y)
        Axe.Y -> Pos(-z, y, x)
        Axe.Z -> Pos(y, -x, z)
    }
    fun rotate(axes: List<Axe>): Pos = axes.fold(this) { pos, axe -> pos.rotate(axe) }

    fun transform(t: Transform): Pos = rotate(t.rotation).shift(t.shift)
    fun transform(ts: List<Transform>): Pos = ts.fold(this) { pos, t -> pos.transform(t) }

    fun dist(other: Pos): Int = abs(x - other.x) + abs(y - other.y) + abs(z - other.z)
}

data class Transform(val rotation: List<Axe>, val shift: Pos)

class Scanner(val beacons: List<Pos>) {
    var transforms = listOf(Transform(listOf(), Pos(0, 0, 0)))
    var pos = Pos(0, 0, 0)

    fun overlap(other: Scanner): Transform? {
        for (b1 in beacons) {
            for (b2 in other.beacons) {
                for (rot in Axe.allRotations()) {
                    val pos = b2.rotate(rot)
                    val shift = Pos(b1.x - pos.x, b1.y - pos.y, b1.z - pos.z)
                    val matches = other.beacons.count {
                        beacons.contains(it.rotate(rot).shift(shift))
                    }
                    if (matches >= 12)
                        return Transform(rot, shift)
                }
            }
        }
        return null
    }
}

fun main() {
    val scanners = File("input").readText().trim().split("\n\n").map {
        Scanner(
            it.split("\n").drop(1).map {
                it.split(",").map(String::toInt).let { Pos(it[0], it[1], it[2]) }
            }
        )
    }

    val todo = mutableListOf(scanners[0])
    val seen = mutableSetOf<Scanner>(scanners[0])
    while (!todo.isEmpty()) {
        val scanner = todo.first()
        todo.removeFirst()

        for (s in scanners) {
            if (seen.contains(s))
                continue
            val transform = scanner.overlap(s)
            if (transform != null) {
                s.pos = transform.shift.transform(scanner.transforms)
                s.transforms = listOf(transform) + scanner.transforms
                todo.add(s)
                seen.add(s)
            }
        }
    }

    val allBeacons = scanners
        .flatMap { s -> s.beacons.map { b -> b.transform(s.transforms) } }
        .toSet()
    println("First: ${allBeacons.size}")

    val second = scanners
        .flatMap { s1 -> scanners.map { s2 -> s1 to s2 } }
        .map { (s1, s2) -> s1.pos.dist(s2.pos) }
        .maxOrNull()!!
    println("Second: $second")
}
