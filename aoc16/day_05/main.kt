import java.io.File
import java.security.MessageDigest
import kotlin.text.Charsets.UTF_8

fun hash(str: String): String = MessageDigest
    .getInstance("MD5")
    .digest(str.toByteArray(UTF_8))
    .joinToString("") { byte -> "%02x".format(byte) }

fun main(args: Array<String>) {
    val input = File("input").readLines()[0].trim()

    var first = "xxxxxxxx".toCharArray()
    var second = "xxxxxxxx".toCharArray()

    val interactive = args.size > 0 && args[0] == "--interactive"

    if (interactive) {
        println("\u001Bc")
        println("\u001B[1;1HFirst: ${first.joinToString("")}")
        println("\u001B[2;1HSecond: ${second.joinToString("")}")
    }

    var i = 0
    var x = 0
    while (first.contains('x') || second.contains('x')) {
        while (true) {
            val hash = hash(input + i.toString())
            i += 1

            if (hash.startsWith("00000")) {
                if (x < 8 && first[x] == 'x') {
                    first[x] = hash[5]
                    x += 1
                    if (interactive)
                        println("\u001B[1;1HFirst: ${first.joinToString("")}")
                }

                val pos = hash[5].code - '0'.code
                if (pos >= 0 && pos < 8 && second[pos] == 'x') {
                    second[pos] = hash[6]
                    if (interactive)
                        println("\u001B[2;1HSecond: ${second.joinToString("")}")
                }

                break
            }
        }
    }
    if (!interactive) {
        println("First: ${first.joinToString("")}")
        println("Second: ${second.joinToString("")}")
    }
}
