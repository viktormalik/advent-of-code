import java.io.File
import kotlin.math.abs

data class Octopus(val x: Int, val y: Int, var energy: Int) {
    var flashed = false
}

fun neighs(o: Octopus, octopuses: List<Octopus>): List<Octopus> =
    octopuses.filter { it != o && abs(o.x - it.x) <= 1 && abs(o.y - it.y) <= 1 }

fun main() {
    val input = File("input").readLines()
    val octopuses = input.withIndex().flatMap { (x, row) ->
        row.withIndex().map { (y, e) -> Octopus(x, y, e.toString().toInt()) }
    }

    var first = 0
    var rounds = 1
    while (true) {
        octopuses.forEach { it.energy += 1 }

        while (octopuses.any { it.energy > 9 && !it.flashed }) {
            for (o in octopuses.filter { it.energy > 9 && !it.flashed }) {
                o.flashed = true
                neighs(o, octopuses).forEach { it.energy += 1 }
            }
        }

        if (octopuses.all { it.flashed })
            break

        for (o in octopuses.filter { it.flashed }) {
            if (rounds <= 100) first += 1
            o.flashed = false
            o.energy = 0
        }

        rounds += 1
    }

    println("First: $first")
    println("Second: $rounds")
}
