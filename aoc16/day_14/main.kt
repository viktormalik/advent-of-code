import java.io.File
import java.security.MessageDigest
import kotlin.text.Charsets.UTF_8

fun hash(str: String): String = MessageDigest
    .getInstance("MD5")
    .digest(str.toByteArray(UTF_8))
    .joinToString("") { byte -> "%02x".format(byte) }

fun stretchHash(str: String): String {
    var result = str
    for (i in 0..2016) {
        result = hash(result)
    }
    return result
}

fun findTriple(str: String): Char? = str
    .windowed(3)
    .filter { it.all { c -> c == it[0] } }
    .firstOrNull()
    ?.first()

fun allFives(str: String): List<Char> = str
    .windowed(5)
    .filter { it.all { c -> c == it[0] } }
    .map { it[0] }

fun lastKeyIndex(salt: String, hashFun: (String) -> String): Int {
    val candidates = mutableMapOf<Int, Char>()
    var keysCnt = 0

    for (i in 0..1000000) {
        val hash = hashFun(salt + i.toString())

        val triple = findTriple(hash)
        if (triple != null) {
            candidates.put(i, triple)
        }

        val fives = allFives(hash)
        for (j in i - 1000..i - 1) {
            val cand = candidates.get(j)
            if (cand != null && fives.any { it == cand }) {
                candidates.remove(j)
                keysCnt += 1
                if (keysCnt == 64) {
                    return j
                }
            }
        }
    }
    return 0
}

fun main() {
    val salt = File("input").readText().trim()

    val first = lastKeyIndex(salt, ::hash)
    println("First: $first")

    val second = lastKeyIndex(salt, ::stretchHash)
    println("Second: $second")
}
