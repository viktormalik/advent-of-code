import std/math
import std/sequtils
import std/strutils

type
  Sector = object
    id: int
    size: int
  Block = object
    sectors: seq[Sector]
    free: int
  Disk = seq[Block]

proc initDisk(input: seq[int]): Disk =
  for i in 0..input.len - 1:
    if i %% 2 == 0:
      result.add(Block(sectors: @[Sector(id: i div 2, size: input[i])], free: 0))
    else:
      result.add(Block(sectors: @[], free: input[i]))

proc first(input: seq[int]): Disk =
  var disk = initDisk(input)
  var front = 0
  var back = disk.len - 1
  while front < back:
    if disk[front].free == 0:
      inc(front)
      continue
    if disk[back].sectors.len == 0:
      dec(back)
      continue
    
    let sector = disk[back].sectors[0]
    let size = min(sector.size, disk[front].free)
    disk[front].sectors.add(Sector(id: sector.id, size: size))
    if size == sector.size:
      disk[front].free -= size
      disk[back].sectors = @[]
      disk[back].free = size
    else:
      disk[front].free = 0
      disk[back].sectors[0].size -= size
  return disk

proc second(input: seq[int]): Disk =
  var disk = initDisk(input)
  for id in countdown(disk.len - 1, 0, 2):
    for b in 0..id - 1:
      let sector = disk[id].sectors[0]
      if disk[b].free >= sector.size:
        disk[b].sectors.add(sector)
        disk[b].free -= sector.size
        disk[id].sectors = @[]
        disk[id].free = sector.size
        break
  return disk

proc checksum(disk: Disk): int =
  var id = 0
  for b in disk:
    for s in b.sectors:
      for i in id..id + s.size - 1:
        result += i * s.id
      id += s.size
    id += b.free

let input = readFile("input").strip.items.toSeq.mapIt(ord(it) - ord('0'))

echo "First: " & $first(input).checksum
echo "Second: " & $second(input).checksum
