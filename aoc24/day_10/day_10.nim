import std/sequtils
import std/sets
import std/strutils

type Pos = tuple
  x, y: int

let map = readFile("input").strip.splitLines

proc neighs(p: Pos): seq[Pos] =
  @[
    (x: p.x + 1, y: p.y),
    (x: p.x - 1, y: p.y),
    (x: p.x, y: p.y + 1),
    (x: p.x, y: p.y - 1)
  ].filterIt(it.x >= 0 and it.x < map.len and it.y >= 0 and it.y < map[0].len and
             ord(map[it.x][it.y]) == ord(map[p.x][p.y]) + 1)

proc score(start: Pos): (int, int) =
  var todo = @[start]
  var unique = initHashSet[Pos]()
  var all = 0
  while todo.len > 0:
    let pos = todo[0]
    todo.delete(0)
    for n in pos.neighs:
      if map[n.x][n.y] == '9':
        unique.incl(n)
        inc(all)
      else:
        todo.add(n)
  return (unique.len, all)

var first, second = 0
for x in 0..map.len - 1:
  for y in 0..map[x].len - 1:
    if map[x][y] == '0':
      let (f, s) = score((x: x, y: y))
      first += f
      second += s

echo "First: " & $first
echo "Second: " & $second
