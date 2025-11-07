import std/sequtils
import std/strutils

type Pos = tuple
  x, y: int

proc enlarge(map: seq[string]): seq[string] =
  for x in 0..map.len - 1:
    result.add("")
    for y in 0..map[x].len - 1:
      case map[x][y]
        of '#': result[x] &= "##"
        of 'O': result[x] &= "[]"
        of '.': result[x] &= ".."
        of '@': result[x] &= "@."
        else: discard

let input = readFile("input").strip.split("\n\n")
let basemap = input[0].splitLines
let largemap = enlarge(basemap)

let moves = input[1].splitWhitespace.join

proc next(pos: Pos, dir: char): Pos =
  case dir
    of 'v': (x: pos.x + 1, y: pos.y)
    of '^': (x: pos.x - 1, y: pos.y)
    of '>': (x: pos.x, y: pos.y + 1)
    of '<': (x: pos.x, y: pos.y - 1)
    else: pos

proc box(map: seq[string], pos: Pos): seq[Pos] =
  case map[pos.x][pos.y]
    of '[': @[pos, pos.next('>')]
    of ']': @[pos.next('<'), pos]
    else: @[pos]

proc next(box: seq[Pos], dir: char): seq[Pos] =
  case dir
    of '<': @[box[0].next('<')]
    of '>': @[box[^1].next('>')]
    else: box.mapIt(it.next(dir))

proc movable(map: seq[string], pos: Pos, dir: char): bool =
  let box = map.box(pos)
  let next = box.next(dir)
  if next.allIt(map[it.x][it.y] == '.'):
    return true
  if next.anyIt(map[it.x][it.y] == '#'):
    return false
  return next.allIt(map[it.x][it.y] == '.' or map.movable(it, dir))

proc move(map: var seq[string], pos: Pos, dir: char): Pos =
  if not map.movable(pos, dir):
    return pos

  result = pos
  let obj = map.box(pos)
  let next = obj.next(dir)

  for n in next:
    if map[n.x][n.y] == 'O' or map[n.x][n.y] == '[' or map[n.x][n.y] == ']':
      discard map.move(n, dir)

  if dir == '>':
    map[next[0].x][next[0].y] = map[obj[^1].x][obj[^1].y]
    if obj.len == 2:
      map[obj[1].x][obj[1].y] = map[obj[0].x][obj[0].y]
    map[obj[0].x][obj[0].y] = '.'
  elif dir == '<':
    map[next[0].x][next[0].y] = map[obj[0].x][obj[0].y]
    if obj.len == 2:
      map[obj[0].x][obj[0].y] = map[obj[1].x][obj[1].y]
    map[obj[^1].x][obj[^1].y] = '.'
  else:
    for i in 0..obj.len - 1:
      map[next[i].x][next[i].y] = map[obj[i].x][obj[i].y]
      map[obj[i].x][obj[i].y] = '.'
  result = next[0]

proc coordinate(box: Pos): int = box.x * 100 + box.y

proc findRobot(map: seq[string]): Pos =
  for x in 0..map.len - 1:
    for y in 0..map[x].len - 1:
      if map[x][y] == '@':
        result = (x: x, y: y)

proc gps(map: seq[string]): int =
  for x in 0..map.len - 1:
    for y in 0..map[x].len - 1:
      if map[x][y] == '[' or map[x][y] == 'O':
        result += coordinate((x: x, y: y))

proc simulate(input: seq[string]): int =
  var map = input
  var robot = map.findRobot
  for dir in moves:
    robot = map.move(robot, dir)
  result = map.gps

echo "First: " & $basemap.simulate
echo "Second: " & $largemap.simulate

