import std/algorithm
import std/sequtils
import std/strutils
import std/sugar

proc monotone(report: seq[int], cond: proc(prev, curr: int): bool): int =
  var prev = report[0]
  for curr in report[1..^1]:
    if cond(prev, curr):
      inc(result)
    else:
      prev = curr

proc increasing(report: seq[int]): int =
  monotone(report, (prev, curr) => curr <= prev or curr > prev + 3)

proc decreasing(report: seq[int]): int =
  monotone(report, (prev, curr) => curr >= prev or curr < prev - 3)

proc safe(report: seq[int]): bool =
  report.increasing == 0 or report.decreasing == 0

proc safeSkip(report: seq[int]): bool =
  report.increasing <= 1 or report.reversed.increasing <= 1 or
  report.decreasing <= 1 or report.reversed.decreasing <= 1

let data = readFile("input").strip.splitLines.mapIt(it.split.map(parseInt))

let first = data.filter(safe).len
echo "First: " & $first

let second = data.filter(safeSkip).len
echo "Second: " & $second
