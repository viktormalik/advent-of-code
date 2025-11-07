import std/math
import std/sequtils
import std/strutils

const rows = 103
const cols = 101

type
  Robot = object
    px, py, vx, vy: int
  Quadrant = object
    sx, ex, sy, ey: int

func parse(line: string): Robot =
  let split = line.split
  let pos = split[0][2..^1].split(',').map(parseInt)
  let vel = split[1][2..^1].split(',').map(parseInt)
  Robot(px: pos[0], py: pos[1], vx: vel[0], vy: vel[1])

func wrap(pos, vel, size: int): int =
  result = pos + vel
  if result < 0: result += size
  elif result >= size: result -= size

func inQuadrant(r: Robot, q: Quadrant): bool =
  r.px >= q.sx and r.px <= q.ex and r.py >= q.sy and r.py <= q.ey

proc inQuadrant(robots: seq[Robot], q: Quadrant): int =
  robots.filterIt(it.inQuadrant(q)).len

proc safety(robots: seq[Robot]): int =
  @[
    Quadrant(sx: 0, ex: cols div 2 - 1, sy: 0, ey: rows div 2 - 1),
    Quadrant(sx: 0, ex: cols div 2 - 1, sy: rows div 2 + 1, ey: rows - 1),
    Quadrant(sx: cols div 2 + 1, ex: cols - 1, sy: 0, ey: rows div 2 - 1),
    Quadrant(sx: cols div 2 + 1, ex: cols - 1, sy: rows div 2 + 1, ey: rows - 1)
  ].mapIt(robots.inQuadrant(it)).prod

proc tree(robots: seq[Robot]): bool =
  # Search for a straight line of length at least 10, that's very likely where
  # the picture is.
  for y in 0..rows - 1:
    var line = 0
    for x in 0..cols - 1:
      if robots.anyIt(it.px == x and it.py == y):
        inc(line)
        if line == 10:
          return true
      else:
        line = 0
  return false

var robots = readFile("input").strip.splitLines.map(parse)

for s in 1..10000:
  for r in robots.mitems:
    r.px = wrap(r.px, r.vx, cols)
    r.py = wrap(r.py, r.vy, rows)
  if s == 100:
    echo "First: " & $robots.safety
  # Starting with time 71, the robots tend to cluster with 101 seconds period,
  # so check only those states.
  if (s - 71) %% 101 == 0 and robots.tree:
    echo "Second: " & $s
    quit(0)
