import std/sets
import std/sequtils
import std/strutils

type
  Dir = enum
    up, right, down, left
  Pos = tuple
    x, y: int

let map = readFile("input").strip.splitLines

proc findStart(): Pos =
  for x in 0..map.len - 1:
    for y in 0..map[x].len - 1:
      if map[x][y] == '^':
        return (x, y)

proc inMap(pos: Pos): bool =
  pos.x >= 0 and pos.x < map.len and pos.y >= 0 and pos.y < map[pos.x].len

proc visit(map: seq[string], start: Pos): HashSet[Pos] =
  var guard = start
  var dir = up
  var visited: HashSet[(Pos, Dir)]
  while guard.inMap:
    if visited.containsOrIncl((guard, dir)):
      return initHashSet[Pos]()
    let next = case dir
      of up: (x: guard.x - 1, y: guard.y)
      of right: (x: guard.x, y: guard.y + 1)
      of down: (x: guard.x + 1, y: guard.y)
      of left: (x: guard.x, y: guard.y - 1)
    if next.inMap and map[next.x][next.y] == '#':
      dir = if dir == left: up else: succ(dir)
    else:
      guard = next
  return toHashSet(visited.toSeq.mapIt(it[0]))

let guard = findStart()
let visited = visit(map, guard)

let first = visited.len
echo "First: " & $first

var second = 0
for v in visited:
  var newMap = map
  newMap[v.x][v.y] = '#'
  if visit(newMap, guard).len == 0:
    inc(second)
echo "Second: " & $second
