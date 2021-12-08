import java.io.File

fun len(message: Iterator<Char>, recursive: Boolean): Long {
    var count: Long = 0
    while (message.hasNext()) {
        count += message.asSequence().takeWhile { it != '(' }.count()
        if (!message.hasNext())
            break

        val marker = message.asSequence().takeWhile { it != ')' }.joinToString("")
        val chars = marker.split("x")[0].toInt()
        val rep = marker.split("x")[1].toInt()

        val subseq = message.asSequence().take(chars)
        if (recursive)
            count += len(subseq.iterator(), true) * rep
        else
            count += subseq.count() * rep
    }
    return count
}

fun main() {
    val message = File("input").readText().filterNot { it.isWhitespace() }

    val first = len(message.iterator(), false)
    println("First: $first")

    val second = len(message.iterator(), true)
    println("Second: $second")
}
