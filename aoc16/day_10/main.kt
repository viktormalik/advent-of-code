import java.io.File

data class Rule(val bot_low: Int?, val out_low: Int?, val bot_high: Int?, val out_high: Int?)

fun main() {
    val instructions = File("input").readLines()

    val bots = mutableMapOf<Int, MutableList<Int>>()
    val rules = mutableMapOf<Int, Rule>()
    val outputs = mutableMapOf<Int, Int>()

    for (inst in instructions) {
        if (inst.startsWith("value")) {
            val v = inst.split(" ")[1].toInt()
            val bot = inst.split(" ")[5].toInt()
            bots.getOrPut(bot) { mutableListOf() }.add(v)
        } else if (inst.startsWith("bot")) {
            val bot = inst.split(" ")[1].toInt()
            val low = inst.split(" ")[6].toInt()
            val high = inst.split(" ")[11].toInt()

            val bot_low = if (inst.split(" ")[5] == "bot") low else null
            val out_low = if (inst.split(" ")[5] == "output") low else null
            val bot_high = if (inst.split(" ")[10] == "bot") high else null
            val out_high = if (inst.split(" ")[10] == "output") high else null

            rules.put(bot, Rule(bot_low, out_low, bot_high, out_high))
        }
    }

    while (bots.filterValues { it.size == 2 }.any()) {
        val bot = bots.filterValues { it.size == 2 }.keys.first()
        val vals = bots.get(bot)!!

        val low = vals.minOrNull()!!
        val high = vals.maxOrNull()!!
        if (low == 17 && high == 61) {
            println("First: $bot")
        }

        val rule = rules.get(bot)!!
        if (rule.bot_low != null)
            bots.getOrPut(rule.bot_low) { mutableListOf() }.add(low)
        else if (rule.out_low != null)
            outputs.put(rule.out_low, low)
        if (rule.bot_high != null)
            bots.getOrPut(rule.bot_high) { mutableListOf() }.add(high)
        else if (rule.out_high != null)
            outputs.put(rule.out_high, high)

        vals.clear()
    }

    val second = outputs.get(0)!! * outputs.get(1)!! * outputs.get(2)!!
    println("Second: $second")
}
