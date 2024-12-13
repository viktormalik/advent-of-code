import std/math
import std/sequtils
import std/strutils

type Machine = object
  ax, ay, bx, by, px, py: int

proc parse(machine: string): Machine =
  let split = machine.splitLines.mapIt(it.split(" "))
  Machine(
    ax: split[0][2][2..^2].parseInt,
    ay: split[0][3][2..^1].parseInt,
    bx: split[1][2][2..^2].parseInt,
    by: split[1][3][2..^1].parseInt,
    px: split[2][1][2..^2].parseInt,
    py: split[2][2][2..^1].parseInt
  )

proc tokens(m: Machine): int =
  let a = (m.px * m.by - m.bx * m.py) / (m.ax * m.by - m.bx * m.ay)
  let b = (float(m.py) - a * float(m.ay)) / float(m.by)

  if floor(a) == a and floor(b) == b:
    return 3 * int(a) + int(b)
  return 0

var machines = readFile("input").strip.split("\n\n").map(parse)
let first = machines.map(tokens).sum
echo "First: " & $first

for m in machines.mitems:
  m.px += 10000000000000
  m.py += 10000000000000
let second = machines.map(tokens).sum
echo "Second: " & $second
