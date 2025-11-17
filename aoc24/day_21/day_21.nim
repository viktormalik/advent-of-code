import std/sequtils
import std/strutils
import std/tables

type
  Pos = tuple
    r, c: int
  Pad = Table[char, Pos]

const numPad = {
  '0': (0, 1),
  'A': (0, 2),
  '1': (1, 0),
  '2': (1, 1),
  '3': (1, 2),
  '4': (2, 0),
  '5': (2, 1),
  '6': (2, 2),
  '7': (3, 0),
  '8': (3, 1),
  '9': (3, 2)
}.toTable

const dirPad = {
  '<': (0, 0),
  'v': (0, 1),
  '>': (0, 2),
  '^': (1, 1),
  'A': (1, 2),
}.toTable

let codes = readFile("input").strip.splitLines

func moveRow(n: int): string =
  if n > 0: '^'.repeat(n) elif n < 0: 'v'.repeat(-n) else: ""

func moveCol(n: int): string =
  if n > 0: '>'.repeat(n) elif n < 0: '<'.repeat(-n) else: ""

func colsFirst(pad: Pad, src: Pos, dst: Pos): bool =
  let dr = dst.r - src.r
  let dc = dst.c - src.c

  # Row/column movement only
  if dr == 0:
    return true
  if dc == 0:
    return false

  # Avoid the blank corners
  if pad == numPad:
    if src.c == 0 and dst.r == 0:
      return true
    if src.r == 0 and dst.c == 0:
      return false
  else:
    if src.c == 0 and dst.r == 1:
      return true
    if src.r == 1 and dst.c == 0:
      return false

  # These are some magic heuristics for both row/column movements
  if dr > 0 and dc > 0:
    return false
  if dr > 0 and dc < 0:
    return true
  if dr < 0 and dc > 0:
    return false
  if dr < 0 and dc < 0:
    return true

proc move(pad: Pad, src: Pos, dst: Pos): string =
  let dr = dst.r - src.r
  let dc = dst.c - src.c
  if pad.colsFirst(src, dst):
    result = moveCol(dc) & moveRow(dr)
  else:
    result = moveRow(dr) & moveCol(dc)
  result &= 'A'

var cache: Table[(int, string), int]

proc length(pad: Pad, level: int, keys: string): int =
  if level == 0:
    return keys.len

  if cache.contains((level, keys)):
    return cache[(level, keys)]

  var chunks: seq[string]
  var cur = 'A'
  for key in keys:
    chunks.add(pad.move(pad[cur], pad[key]))
    cur = key

  result = chunks.mapIt(dirPad.length(level - 1, it)).foldl(a + b)
  cache[(level, keys)] = result

proc complexity(code: string, levels: int): int =
  numPad.length(levels + 1, code) * parseInt(code[0..^2])

let first = codes.mapIt(complexity(it, 2)).foldl(a + b)
echo "First: " & $first
let second = codes.mapIt(complexity(it, 25)).foldl(a + b)
echo "Second: " & $second
