import std/sequtils
import std/strutils

func parseLine(line: string): seq[int] =
  line.mapIt(ord(it) - ord('0'))

proc joltage(bank: seq[int], digits: int): int =
  var prev = -1
  for d in 0 .. digits - 1:
    let i = bank[prev + 1 .. ^(digits - d)].maxIndex
    result = result * 10 + bank[prev + i + 1]
    prev = prev + i + 1

let banks = readFile("input").strip.splitLines.map(parseLine)

let first = banks.mapIt(joltage(it, 2)).foldl(a + b)
echo "First: " & $first
let second = banks.mapIt(joltage(it, 12)).foldl(a + b)
echo "Second: " & $second
