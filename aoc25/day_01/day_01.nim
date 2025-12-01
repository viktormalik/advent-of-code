import std/strutils

let rotations = readFile("input").strip.splitLines

var dial = 50
var first, second = 0
for r in rotations:
  let n = r[1..^1].parseInt
  for i in 1..n:
    dial = (dial + (if r[0] == 'L': -1 else: 1)) mod 100
    second += cast[int](dial == 0)
  first += cast[int](dial == 0)

echo "First: " & $first
echo "Second: " & $second
