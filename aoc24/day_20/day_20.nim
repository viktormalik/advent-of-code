import std/sequtils
import std/sets
import std/strutils
import std/tables
import ../../utils/nim/pathutils

let track = readFile("input").strip.splitLines

var start, finish: Pos
for r in 0..track.len - 1:
  for c in 0..track[r].len - 1:
    if track[r][c] == 'S':
      start = (r, c)
    elif track[r][c] == 'E':
      finish = (r, c)

proc inBounds(p: Pos): bool =
  p.x >= 0 and p.x < track.len and p.y >= 0 and p.y < track[0].len

proc neighs(p: Pos): seq[Pos] =
  p.fourNeighs.filterIt(track[it.x][it.y] != '#')

proc cheatNeighs(p: Pos): seq[Pos] =
  p.fourNeighs.filter(inBounds)

proc onPath(p: Pos): bool = p.inBounds and track[p.x][p.y] != '#'

let shortest = shortest(start, finish, neighs)
var shortestDst = initTable[Pos, int]()
for i, p in shortest.ps:
  shortestDst[p] = i

proc cheats(i: int, size: int): seq[int] =
  let p = shortest.ps[i]
  for t in reachable(p, size, cheatNeighs).items.toSeq.filter(onPath):
    let l = abs(p.x - t.x) + abs(p.y - t.y)
    let ti = shortestDst[t]
    if ti - i - l > 0:
      result.add(ti - i - l)

var first = 0
var second = 0
for i in 0..shortest.ps.len - 1:
  first += cheats(i, 2).filterIt(it >= 100).len
  second += cheats(i, 20).filterIt(it >= 100).len
echo "First: " & $first
echo "Second: " & $second
