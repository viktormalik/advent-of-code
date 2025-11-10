import std/sequtils
import std/strutils
import std/sugar
import ../../utils/nim/pathutils

func parse(byte: string): Pos =
  let split = byte.split(',').map(parseInt)
  (x: split[0], y: split[1])

let bytes = readFile("input").strip.splitLines.map(parse)

var memory: array[0..70, array[0..70, int]]
for b in 0..bytes.len - 1:
  memory[bytes[b].x][bytes[b].y] = b + 1

proc inMem(p: Pos): bool = p.x >= 0 and p.x <= 70 and p.y >= 0 and p.y <= 70

proc hasByte(p: Pos, bytesCnt: int): bool =
  memory[p.x][p.y] > 0 and memory[p.x][p.y] <= bytesCnt

proc neighs(bytesCnt: int): NeighFunc =
  return proc (p: Pos): seq[Pos] =
    p.fourNeighs.filterIt(it.inMem and not it.hasByte(bytesCnt))

proc onPath(byte: Pos, path: Path): bool = path.ps.find(byte) != -1

let start = (x: 0, y: 0)
let target = (x: 70, y: 70)
var shortest = shortest(start, target, neighs(1024))

echo "First: " & $shortest.dist

for b in 1025..bytes.len - 1:
  let byte = bytes[b - 1]
  if not byte.onPath(shortest):
    continue
  shortest = shortest(start, target, neighs(b))
  if shortest.dist == -1:
    echo "Second: " & $byte.x & "," & $byte.y
    break
