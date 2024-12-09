import std/algorithm
import std/math
import std/sequtils
import std/strutils

type
  Equation = object
    res: int
    nums: seq[int]
  Op = enum
    sum, mul, concat

func parse(line: string): Equation =
  let split = line.split(": ")
  let nums = split[1].split(' ').map(parseInt)
  Equation(res: split[0].parseInt, nums: nums)

func concat(a, b: int): int =
  result = a
  var order = b
  while order div 10 != 0:
    result *= 10
    order = order div 10
  return result * 10 + b

func op(a: int, b: (int, Op)): int =
  case b[1]
    of sum: a + b[0]
    of mul: a * b[0]
    of concat: concat(a, b[0])

func valid(eq: Equation, ops: seq[Op]): bool =
  for opseq in ops.repeat(eq.nums.len - 1).product:
    let res = eq.nums[1..^1].zip(opseq).foldl(a.op(b), eq.nums[0])
    if res == eq.res:
      return true
  return false

func calibrate(equations: seq[Equation], ops: seq[Op]): int =
  equations.filterIt(it.valid(ops)).mapIt(it.res).sum

let equations = readFile("input").strip.splitLines.map(parse)

let first = calibrate(equations, @[sum, mul])
echo "First: " & $first

let second = calibrate(equations, @[sum, mul, concat])
echo "Second: " & $second
