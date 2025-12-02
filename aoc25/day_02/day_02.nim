import std/sequtils
import std/strutils

func parseRange(line: string): (int, int) =
  let split = line.split("-").map(parseInt)
  (split[0], split[1])

let ranges = readFile("input").strip.split(",").map(parseRange)

proc invalid(n: string, seqSize: int): bool =
  if n.len mod seqSize != 0:
    return false
  for i in 1 ..< n.len div seqSize:
      if n[0 ..< seqSize] != n[i * seqSize ..< (i + 1) * seqSize]:
        return false
  true

proc invalidFirst(n: string): bool =
  let s = n.len div 2
  if n.len == 2 * s:
    return invalid(n, s)
  false

proc invalidSecond(n: string): bool =
  for s in 1..n.len div 2:
    if invalid(n, s):
      return true
  false

proc invalidSum(range: (int, int), invalidFn: proc (n: string): bool): int =
  for i in range[0]..range[1]:
    if invalidFn($i):
      result += i

let first = ranges.mapIt(it.invalidSum(invalidFirst)).foldl(a + b)
echo "First: " & $first
let second = ranges.mapIt(it.invalidSum(invalidSecond)).foldl(a + b)
echo "Second: " & $second
