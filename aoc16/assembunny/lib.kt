package assembunny

class Assembunny(instructions: Array<Inst>) {
    data class Inst(var name: String, val op1: String, val op2: String?) {
        companion object {
            fun parse(line: String) = line.split(" ").let {
                Inst(it[0], it[1], if (it.size == 3) it[2] else null)
            }
        }
    }

    val regs = ('a'..'d').map { it to 0 }.toMap().toMutableMap()
    val instructions = instructions
    var ip = 0
    var stdout = StringBuilder()

    constructor (instructions: List<String>) :
        this(instructions.map { Inst.parse(it) }.toTypedArray())

    constructor (instructions: List<String>, initRegs: Map<Char, Int>) : this(instructions) {
        for ((r, v) in initRegs)
            regs.put(r, v)
    }

    fun opVal(op: String): Int = op.toIntOrNull() ?: regs[op[0]]!!

    fun run(stdoutLen: Int? = null): Int {
        while (ip < instructions.size) {
            val inst = instructions[ip]

            when (inst.name) {
                "cpy" -> regs.put(inst.op2!![0], opVal(inst.op1))
                "inc" -> regs.merge(inst.op1[0], 1, Int::plus)
                "dec" -> regs.merge(inst.op1[0], 1, Int::minus)
                "tgl" -> {
                    val x = ip + opVal(inst.op1)
                    if (x >= 0 && x < instructions.size) {
                        val toggleInst = instructions[x]
                        when (toggleInst.name) {
                            "inc" -> toggleInst.name = "dec"
                            "dec" -> toggleInst.name = "inc"
                            "tgl" -> toggleInst.name = "inc"
                            "jnz" -> toggleInst.name = "cpy"
                            "cpy" -> toggleInst.name = "jnz"
                        }
                    }
                }
                "out" -> {
                    stdout.append(opVal(inst.op1))
                    if (stdoutLen != null && stdoutLen == stdout.length)
                        return regs['a']!!
                }
            }

            if (inst.name == "jnz" && opVal(inst.op1) != 0)
                ip += opVal(inst.op2!!)
            else if (inst.name == "tgl") {
                // Acceleration
                regs.merge('a', regs['b']!!, Int::times)
                regs.merge('b', 1, Int::minus)
                regs.merge('c', 2, Int::minus)
            } else
                ip += 1
        }
        return regs['a']!!
    }
}
