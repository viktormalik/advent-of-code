import std/algorithm
import std/math
import std/sequtils
import std/strutils

let input = readFile("input").strip.split("\n\n")

let rules = input[0].splitLines.mapIt(it.split('|').map(parseInt))
let lists = input[1].splitLines.mapIt(it.split(',').map(parseInt))

proc cmp(x, y: int): int =
  if rules.find(@[x, y]) != -1: return -1
  if rules.find(@[y, x]) != -1: return 1
  return 0

let first = lists.mapIt(if it.isSorted(cmp): it[it.len div 2] else: 0).sum
echo "First: " & $first

let second = lists.mapIt(if it.isSorted(cmp): 0 else: it.sorted(cmp)[it.len div 2]).sum
echo "Second: " & $second
