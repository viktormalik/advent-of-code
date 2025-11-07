import std/sets

type
  Pos* = tuple
    x, y: int

  Path* = object
    ps*: seq[Pos]
    dist*: int

  NeighFunc* = proc (p: Pos): seq[Pos]

func last*(path: Path): Pos = path.ps[^1]

proc step*(path: Path, pos: Pos): Path =
  Path(ps: path.ps & @[pos], dist: path.dist + 1)

proc fourNeighs*(p: Pos): seq[Pos] =
  @[
    (x: p.x + 1, y: p.y),
    (x: p.x - 1, y: p.y),
    (x: p.x, y: p.y + 1),
    (x: p.x, y: p.y - 1)
  ]

proc shortest*(src: Pos, dest: Pos, neighs: NeighFunc): Path =
  var todo = @[Path(ps: @[src], dist: 0)]
  var seen = toHashSet([src])

  while todo.len > 0:
    let path = todo[0]
    todo.delete(0)

    for n in path.last.neighs:
      if not seen.contains(n):
        let newPath = path.step(n)
        if newPath.last == dest:
          return newPath
        todo.add(newPath)
        seen.incl(n)
  return Path(ps: @[], dist: -1)

proc reachable*(src: Pos, steps: int, neighs: NeighFunc): HashSet[Pos] =
  var todo = @[Path(ps: @[src], dist: 0)]
  result = toHashSet([src])

  while todo.len > 0:
    let path = todo[0]
    todo.delete(0)

    for n in path.last.neighs:
      if not result.contains(n):
        let newPath = path.step(n)
        if newPath.dist > steps:
          return
        todo.add(newPath)
        result.incl(n)
