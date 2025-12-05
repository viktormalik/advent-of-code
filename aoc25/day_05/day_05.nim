import std/sequtils
import std/strutils

func parseRange(line: string): (int, int) =
  let split = line.split("-").map(parseInt)
  (split[0], split[1])

let input = readFile("input").strip.split("\n\n")
var fresh = input[0].splitLines.map(parseRange)
let ingredients = input[1].splitLines.map(parseInt)

func contains(r: (int, int), ing: int): bool =
  ing >= r[0] and ing <= r[1]

proc isFresh(ing: int): bool =
  fresh.anyIt(it.contains(ing))

let first = ingredients.filter(isFresh).len
echo "First: " & $first

func isValid(r: (int, int)): bool =
  r[0] != -1 and r[1] != -1

func includes(r1, r2: (int, int)): bool =
  r1.isValid and r2.isValid and r1[0] <= r2[0] and r1[1] >= r2[1]

func overlaps(r1, r2: (int, int)): bool =
  r1.isValid and r2.isValid and r1[0] <= r2[0] and r1[1] >= r2[0]

var prev = 0
while prev != fresh.len:
  prev = fresh.len
  for i in 0..<fresh.len:
    for j in 0..<fresh.len:
      if i == j:
        continue
      if fresh[i].includes(fresh[j]):
        fresh[j] = (-1, -1)
      elif fresh[j].includes(fresh[i]):
        fresh[i] = (-1, -1)
      elif fresh[i].overlaps(fresh[j]):
        fresh[i][1] = fresh[j][1]
        fresh[j] = (-1, -1)
      elif fresh[j].overlaps(fresh[i]):
        fresh[j][1] = fresh[i][1]
        fresh[i] = (-1, -1)
  fresh = fresh.filter(isValid)

let second = fresh.mapIt(it[1] - it[0] + 1).foldl(a + b)
echo "Second: " & $second
