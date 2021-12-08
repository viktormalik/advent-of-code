import java.io.File

data class Room(val name: String, val id: Int, val checksum: String)

fun parseRoom(line: String): Room {
    val regex = "(.*)-(\\d+)\\[(.*)\\]".toRegex()
    val match = regex.find(line)

    val name = match?.groupValues?.get(1)
    val id = match?.groupValues?.get(2)?.toInt()
    val checksum = match?.groupValues?.get(3)

    return Room(name!!, id!!, checksum!!)
}

fun isRoom(room: Room): Boolean {
    var occurrences = mutableMapOf<Char, Int>()
    for (c in room.name) {
        if (c != '-') {
            var v = occurrences.getOrDefault(c, 0)
            occurrences.put(c, v + 1)
        }
    }

    val checksum = occurrences
        .toList()
        .sortedWith(compareBy({ (_, value) -> -value }, { (c, _) -> c }))
        .map { (c, _) -> c }
        .slice(0..4)
        .toCharArray()
        .joinToString("")

    return checksum == room.checksum
}

fun decrypt(room: Room): String = room
    .name
    .map { if (it == '-') it else ((it.code - 'a'.code + room.id) % 26 + 'a'.code).toChar() }
    .toCharArray()
    .joinToString("")

fun main() {
    val rooms = File("input").readLines().map { parseRoom(it) }

    val first = rooms.filter { isRoom(it) }.map { it.id }.sum()
    println("First: $first")

    val second = rooms.find { decrypt(it) == "northpole-object-storage" }?.id
    println("Second: $second")
}
