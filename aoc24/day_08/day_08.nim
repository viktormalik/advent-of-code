import std/sets
import std/strutils
import std/tables

type Pos = tuple
  x, y: int

let map = readFile("input").strip.splitLines

var antennas = initTable[char, seq[Pos]]()
for x in 0..map.len - 1:
  for y in 0..map[x].len - 1:
    if map[x][y] != '.':
      if not antennas.hasKey(map[x][y]):
        antennas[map[x][y]] = @[]
      antennas[map[x][y]].add((x: x, y: y))

var first = initHashSet[(int, int)]()
var second = initHashSet[(int, int)]()
for c, ps in antennas:
  for p1 in ps:
    for p2 in ps:
      if p1 == p2:
        continue
      let dx = p1.x - p2.x
      let dy = p1.y - p2.y
      var a = p1
      while a.x >= 0 and a.x < map.len and a.y >= 0 and a.y < map[0].len:
        second.incl(a)
        if a.x - p1.x == dx and a.y - p1.y == dy:
          first.incl(a)
        a.x += dx
        a.y += dy

echo "First: " & $first.len
echo "Second: " & $second.len
