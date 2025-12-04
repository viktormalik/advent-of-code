import std/sequtils
import std/strutils
import ../../utils/nim/pathutils

var grid = readFile("input").strip.splitLines

func contains(grid: seq[string], p: Pos): bool =
  p.x >= 0 and p.x < grid.len and p.y >= 0 and p.y < grid[p.x].len

func accessible(grid: seq[string], p: Pos): bool =
  grid[p.x][p.y] == '@' and
    p.eightNeighs.filterIt(grid.contains(it) and grid[it.x][it.y] == '@').len < 4

func removable(grid: seq[string]): seq[Pos] =
  for x in 0..<grid.len:
    for y in 0..<grid[x].len:
      if grid.accessible((x, y)):
        result.add((x, y))

let first = grid.removable.len
echo "First: " & $first

var second = 0
while true:
  let rem = grid.removable
  if rem.len == 0:
    break
  second += rem.len
  for p in rem:
    grid[p.x][p.y] = '.'
echo "Second: " & $second
