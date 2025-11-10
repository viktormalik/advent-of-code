import std/bitops
import std/math
import std/sequtils
import std/strutils

let input = readFile("input").strip.split("\n\n")
let regsInput = input[0].splitLines.mapIt(it.split()[2].parseInt)
let progInput = input[1].split()[1]

let prog = progInput.split(',').map(parseInt)
var regs: array[4..6, int] = [0, 0, 0]

proc combo(op: int): int =
  case op
    of 0, 1, 2, 3: op
    of 4, 5, 6: regs[op]
    else: 0

proc run(a, b, c: int): string =
  regs = [a, b, c]
  var ip = 0
  var output: seq[int]
  while ip < prog.len:
    let op = prog[ip + 1]
    case prog[ip]
      of 0: regs[4] = regs[4] div (2 ^ combo(op))
      of 1: regs[5] = regs[5].bitxor(op)
      of 2: regs[5] = combo(op) %% 8
      of 3:
        if regs[4] != 0: ip = op else: ip += 2
        continue
      of 4: regs[5] = regs[5].bitxor(regs[6])
      of 5: output.add(combo(op) %% 8)
      of 6: regs[5] = regs[4] div (2 ^ combo(op))
      of 7: regs[6] = regs[4] div (2 ^ combo(op))
      else: discard
    ip += 2
  return output.mapIt($it).join(",")

proc match(res: string, digit: int): bool =
  for d in countup(digit, (prog.len - 1) * 2, 2):
    if res[d] != progInput[d]:
      return false
  return true

let first = run(regsInput[0], regsInput[1], regsInput[2])
echo "First: " & $first

var res = ""
var starts = @[8 ^ (prog.len - 1)]
var step = starts[0]
var digit = (prog.len - 1) * 2
while true:
  var nextStarts: seq[int]
  for start in starts:
    for a in countup(start, start + (8 * step) - 1, step):
      var r = run(a, regsInput[1], regsInput[2])
      if match(r, digit):
        if digit == 0 and r == progInput:
          echo "Second: " & $a
          quit(0)
        nextStarts.add(a)
  starts = nextStarts
  step = step div 8
  digit -= 2

