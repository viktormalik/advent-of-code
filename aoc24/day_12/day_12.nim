import std/math
import std/sets
import std/sequtils
import std/strutils

type
  Plant = tuple
    x, y: int
  Region = HashSet[Plant]

let map = readFile("input").strip.splitLines

proc inMap(p: Plant): bool =
  p.x >= 0 and p.x < map.len and p.y >= 0 and p.y < map[0].len

proc neighs(p: Plant): seq[Plant] =
  @[
    (x: p.x - 1, y: p.y),
    (x: p.x + 1, y: p.y),
    (x: p.x, y: p.y - 1),
    (x: p.x, y: p.y + 1),
  ].filter(inMap)

proc perimeter(p: Plant): int =
  4 - p.neighs.filterIt(map[p.x][p.y] == map[it.x][it.y]).len

proc outerCorner(p, n1, n2: Plant): bool =
  (not n1.inMap or map[p.x][p.y] != map[n1.x][n1.y]) and
  (not n2.inMap or map[p.x][p.y] != map[n2.x][n2.y])

proc innerCorner(p, n1, n2, nx: Plant): bool =
  n1.inMap and n2.inMap and nx.inMap and map[p.x][p.y] != map[nx.x][nx.y] and
  map[p.x][p.y] == map[n1.x][n1.y] and map[p.x][p.y] == map[n2.x][n2.y]

proc corners(p: Plant): int =
  let up = (x: p.x - 1, y: p.y)
  let down = (x: p.x + 1, y: p.y)
  let left = (x: p.x, y: p.y - 1)
  let right = (x: p.x, y: p.y + 1)
  let upleft = (x: p.x - 1, y: p.y - 1)
  let upright = (x: p.x - 1, y: p.y + 1)
  let downleft = (x: p.x + 1, y: p.y - 1)
  let downright = (x: p.x + 1, y: p.y + 1)

  result += @[(up, left), (up, right), (down, left), (down, right)]
    .filterIt(outerCorner(p, it[0], it[1])).len

  result += @[
    (up, left, upleft),
    (up, right, upright),
    (down, left, downleft),
    (down, right, downright)
  ].filterIt(innerCorner(p, it[0], it[1], it[2])).len

proc area(region: Region): int = region.len
proc perimeter(region: Region): int = region.toSeq.map(perimeter).sum
proc sides(region: Region): int = region.toSeq.map(corners).sum

var visited = initHashSet[Plant]()
var regions: seq[Region]
for x in 0..map.len - 1:
  for y in 0..map[x].len - 1:
    let start = (x: x, y: y)
    if visited.contains(start):
      continue

    var region = toHashSet([start])
    var todo = @[start]
    while todo.len > 0:
      let p = todo[0]
      todo.delete(0)
      for n in p.neighs:
        if not visited.contains(n) and map[x][y] == map[n.x][n.y]:
          region.incl(n)
          visited.incl(n)
          todo.add(n)

    regions.add(region)

let first = regions.mapIt(it.area * it.perimeter).sum
echo "First: " & $first
let second = regions.mapIt(it.area * it.sides).sum
echo "Second: " & $second
