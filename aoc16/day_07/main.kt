import java.io.File

data class IPv7(val supernet: List<String>, val hypernet: List<String>)

fun parse_ip(line: String): IPv7 {
    val split = line.split("[\\[\\]]".toRegex())
    return IPv7(
        split.filterIndexed { i, _ -> (i % 2) == 0 },
        split.filterIndexed { i, _ -> (i % 2) == 1 },
    )
}

fun has_abba(s: String): Boolean {
    return s
        .indices
        .any { i -> i >= 3 && s[i] == s[i - 3] && s[i - 1] == s[i - 2] && s[i] != s[i - 1] }
}

fun has_tls(ip: IPv7): Boolean =
    ip.supernet.any { has_abba(it) } && ip.hypernet.none { has_abba(it) }

fun has_ssl(ip: IPv7): Boolean =
    ip.supernet.any { seq ->
        seq.withIndex().any { (i, c) ->
            i > 0 && i < seq.length - 1 && seq[i - 1] == seq[i + 1] && c != seq[i - 1] &&
                ip.hypernet.any { it.contains(charArrayOf(c, seq[i + 1], c).joinToString("")) }
        }
    }

fun main() {
    val ips = File("input").readLines().map { parse_ip(it) }

    val first = ips.filter { has_tls(it) }.count()
    println("First: $first")

    val second = ips.filter { has_ssl(it) }.count()
    println("Second: $second")
}
