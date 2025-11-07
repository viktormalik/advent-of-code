import std/algorithm
import std/sequtils
import std/sets
import std/strutils
import std/tables
import ../../utils/nim/pathutils

let map = readFile("input").strip.splitLines

proc find(c: char): Pos =
  for x in 0..map.len - 1:
    for y in 0..map[x].len - 1:
      if map[x][y] == c:
        return (x: x, y: y)
  return (x: -1, y: -1)

proc neighs(pos: Pos): seq[Pos] =
  pos.fourNeighs.filterIt(map[it.x][it.y] != '#')

proc cmp(p1, p2: Path): int =
  system.cmp(p1.dist, p2.dist)

let start = find('S')
let target = find('E')

var todo = @[Path(ps: @[start], dist: 0)]
var visited = initTable[Pos, int]()
var best = -1
var second = initHashSet[Pos]()
while todo.len > 0:
  let path = todo[0]
  todo.delete(0)

  if best >= 0 and path.dist >= best:
    continue

  for n in path.last.neighs:
    if n notin path.ps:
      var newPath = path.step(n)
      if path.ps.len == 1 and n.x != path.last.x:
        newPath.dist += 1000
      elif path.ps.len > 1 and 
           ((n.x == path.last.x and n.x != path.ps[^2].x) or
            (n.y == path.last.y and n.y != path.ps[^2].y)):
        newPath.dist += 1000

      if n == target:
        if best == -1:
          echo "First: " & $newPath.dist
          best = newPath.dist
        if newPath.dist == best:
          second = second.union(newPath.ps.toHashSet())
      else:
        if not visited.contains(n) or newPath.dist <= visited[n] + 1000:
          if not visited.contains(n) or newPath.dist < visited[n]:
            visited[n] = newPath.dist
          todo.add(newPath)
          todo.sort(cmp)

echo "Second: " & $second.len
