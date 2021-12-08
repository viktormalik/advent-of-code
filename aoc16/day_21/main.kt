import java.io.File

interface Op {
    fun apply(pass: CharArray): CharArray
    fun inverse(): Op
}

data class SwapPos(val x: Int, val y: Int) : Op {
    override fun apply(pass: CharArray): CharArray = pass.apply {
        this[x] = this[y].also { this[y] = this[x] }
    }
    override fun inverse() = this
}

data class SwapLet(val x: Char, val y: Char) : Op {
    override fun apply(pass: CharArray): CharArray = pass.apply {
        pass[indexOf(x)] = pass[indexOf(y)].also { pass[indexOf(y)] = pass[indexOf(x)] }
    }
    override fun inverse() = this
}

data class RotateLeft(val x: Int) : Op {
    override fun apply(pass: CharArray): CharArray = (pass.drop(x) + pass.take(x)).toCharArray()
    override fun inverse() = RotateRight(x)
}

data class RotateRight(val x: Int) : Op {
    override fun apply(pass: CharArray): CharArray =
        (pass.drop(pass.size - x) + pass.take(pass.size - x)).toCharArray()
    override fun inverse() = RotateLeft(x)
}

data class RotateBased(val x: Char) : Op {
    override fun apply(pass: CharArray): CharArray {
        var n = pass.indexOf(x)
        n = (n + if (n >= 4) 2 else 1) % pass.size
        return RotateRight(n).apply(pass)
    }
    override fun inverse() = RotateBasedInv(x)
}

data class RotateBasedInv(val x: Char) : Op {
    override fun apply(pass: CharArray): CharArray {
        return when (pass.indexOf(x)) {
            0 -> RotateLeft(1).apply(pass)
            1 -> RotateLeft(1).apply(pass)
            2 -> RotateLeft(6).apply(pass)
            3 -> RotateLeft(2).apply(pass)
            4 -> RotateLeft(7).apply(pass)
            5 -> RotateLeft(3).apply(pass)
            6 -> NoOp().apply(pass)
            7 -> RotateLeft(4).apply(pass)
            else -> NoOp().apply(pass)
        }
    }
    override fun inverse() = RotateBased(x)
}

data class Reverse(val x: Int, val y: Int) : Op {
    override fun apply(pass: CharArray) =
        (pass.take(x) + pass.slice(x..y).reversed() + pass.drop(y + 1)).toCharArray()
    override fun inverse() = this
}

data class Move(val x: Int, val y: Int) : Op {
    override fun apply(pass: CharArray): CharArray {
        val tmp = pass.toMutableList()
        val elem = tmp.removeAt(x)
        tmp.add(y, elem)
        return tmp.toCharArray()
    }
    override fun inverse() = Move(y, x)
}

class NoOp() : Op {
    override fun apply(pass: CharArray): CharArray = pass
    override fun inverse() = this
}

fun parse(inst: String): Op =
    with(inst.split(" ")) {
        if (inst.startsWith("swap position")) SwapPos(this[2].toInt(), this[5].toInt())
        else if (inst.startsWith("swap letter")) SwapLet(this[2][0], this[5][0])
        else if (inst.startsWith("rotate left")) RotateLeft(this[2].toInt())
        else if (inst.startsWith("rotate right")) RotateRight(this[2].toInt())
        else if (inst.startsWith("rotate based")) RotateBased(this[6][0])
        else if (inst.startsWith("reverse")) Reverse(this[2].toInt(), this[4].toInt())
        else if (inst.startsWith("move")) Move(this[2].toInt(), this[5].toInt())
        else NoOp()
    }

fun main() {
    val initPass = "abcdefgh".toCharArray()
    var insts = File("input").readLines()

    val first = insts.fold(initPass) { pass, inst -> parse(inst).apply(pass) }
    println("First: ${first.joinToString("")}")

    val targetPass = "fbgdceah".toCharArray()
    val second = insts.reversed().fold(targetPass) { pass, inst ->
        parse(inst).inverse().apply(pass)
    }
    println("Second: ${second.joinToString("")}")
}
