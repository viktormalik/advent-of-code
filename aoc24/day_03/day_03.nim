import std/math
import std/nre
import std/sequtils
import std/strutils

func mul(s: string): int = s[4..^2].split(',').map(parseInt).prod

let regex = re"(mul\([0-9]*,[0-9]*\)|do(n't)?\(\))"

let input = readFile("input").strip

let first = input.findAll(regex).filterIt(it.startsWith("mul")).map(mul).sum

var second = 0
var enabled = true
for match in input.findAll(regex):
  if match.startsWith("mul") and enabled:
      second += match.mul
  else:
    enabled = match.startsWith("do()")

echo "First: " & $first
echo "Second: " & $second
