import std/math
import std/sequtils
import std/strutils
import std/tables

func digits(n: int): int =
  var res = n
  while res >= 10:
    res = res div 10
    inc(result)
  inc(result)

var cache = initTable[(int, int), int]()

proc count(stone: int, steps: int): int =
  if steps == 0:
    return 1
  if cache.contains((stone, steps)):
    return cache[(stone, steps)]

  if stone == 0:
    result = count(1, steps - 1)
  elif stone.digits %% 2 == 0:
    let divisor = 10 ^ (stone.digits div 2)
    result = count(stone div divisor, steps - 1) +
             count(stone %% divisor, steps - 1)
  else:
    result = count(stone * 2024, steps - 1)

  cache[(stone, steps)] = result

let stones = readFile("input").strip.split.map(parseInt)

let first = stones.mapIt(count(it, 25)).sum
echo "First: " & $first
let second = stones.mapIt(count(it, 75)).sum
echo "Second: " & $second
