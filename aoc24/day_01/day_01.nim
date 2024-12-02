import std/algorithm
import std/math
import std/sequtils
import std/strutils

let list =
  readFile("input").strip.splitLines.mapIt(it.splitWhitespace.map(parseInt))

let left = list.mapIt(it[0]).sorted(system.cmp)
let right = list.mapIt(it[1]).sorted(system.cmp)

let first = left.zip(right).mapIt(abs(it[0] - it[1])).sum
echo "First: " & $first

let second = left.zip(right).mapIt(it[0] * right.count(it[0])).sum
echo "Second: " & $second
