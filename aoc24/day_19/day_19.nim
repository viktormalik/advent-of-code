import std/math
import std/sequtils
import std/strutils
import std/tables

let input = readFile("input").strip.split("\n\n")

let towels = input[0].split(", ")
let patterns = input[1].splitLines

var cache = initTable[string, int]()

proc possible(pat: string): int =
  if pat == "":
    return 1
  if pat in cache:
    return cache[pat]
  for t in towels:
    if pat.startsWith(t):
      result += possible(pat[t.len..^1])
  cache[pat] = result
  return result

let first = patterns.filterIt(it.possible > 0).len
echo "First: " & $first
let second = patterns.mapIt(it.possible).sum
echo "Second: " & $second
