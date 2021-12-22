import java.io.File
import kotlin.math.max

enum class Player {
    P1, P2;

    fun opposite(): Player = if (this == P1) P2 else P1
}

data class Game(
    var p1Pos: Int,
    var p2Pos: Int,
    var p1Score: Int = 0,
    var p2Score: Int = 0,
    val winningScore: Int,
    var playing: Player = Player.P1,
) {
    var rolls = 0

    fun clone(): Game = Game(p1Pos, p2Pos, p1Score, p2Score, winningScore, playing)

    fun round(dice: List<Int>) {
        rolls += dice.size
        if (playing == Player.P1) {
            p1Pos = (p1Pos + dice.sum()) % 10
            p1Score += p1Pos + 1
        } else {
            p2Pos = (p2Pos + dice.sum()) % 10
            p2Score += p2Pos + 1
        }
        playing = playing.opposite()
    }

    fun end(): Boolean = p1Score >= winningScore || p2Score >= winningScore

    fun winner(): Player = playing.opposite()
    fun looserScore(): Int = if (playing == Player.P1) p1Score else p2Score
}

class QuantumGame {
    var activeGames = mapOf<Game, Long>()
    val endedGames = mutableMapOf<Game, Long>()

    constructor (initGame: Game) { activeGames = mapOf(initGame to 1) }

    fun round(roll: Map<Int, Long>) {
        val newGames = mutableMapOf<Game, Long>()
        for ((game, gameCnt) in activeGames) {
            for ((rollVal, rollCnt) in roll) {
                val newGame = game.clone()
                newGame.round(listOf(rollVal))
                if (newGame.end())
                    endedGames.merge(newGame, rollCnt * gameCnt, Long::plus)
                else
                    newGames.merge(newGame, rollCnt * gameCnt, Long::plus)
            }
        }
        activeGames = newGames
    }

    fun end(): Boolean = activeGames.isEmpty()

    fun p1Wins(): Long = endedGames.filter { it.key.winner() == Player.P1 }.values.sum()
    fun p2Wins(): Long = endedGames.filter { it.key.winner() == Player.P2 }.values.sum()
}

fun normalDie(): Sequence<Int> = sequence { while (true) { yieldAll(1..100) } }
fun quantumDie(): Map<Int, Long> = mapOf(3 to 1, 4 to 3, 5 to 6, 6 to 7, 7 to 6, 8 to 3, 9 to 1)

fun main() {
    val input = File("input").readLines()
    val p1Pos = input[0][28].toString().toInt() - 1
    val p2Pos = input[1][28].toString().toInt() - 1

    val normalGame = Game(p1Pos, p2Pos, winningScore = 1000)

    for (nums in normalDie().chunked(3)) {
        normalGame.round(nums)
        if (normalGame.end()) {
            println("First: ${normalGame.looserScore() * normalGame.rolls}")
            break
        }
    }

    var quantumGame = QuantumGame(Game(p1Pos, p2Pos, winningScore = 21))
    while (!quantumGame.end()) {
        quantumGame.round(quantumDie())
    }

    val second = max(quantumGame.p1Wins(), quantumGame.p2Wins())
    println("Second: $second")
}
