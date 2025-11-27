import std/algorithm
import std/sequtils
import std/strutils

type Kind = enum
  Key, Lock

func getHeight(rows: seq[string], col: int): int =
  rows.mapIt(it[col]).count('#') - 1

func parseObject(obj: string): (Kind, seq[int]) =
  let rows = obj.splitLines
  let kind = if rows[0] == "#####": Lock else: Key
  let heights = toSeq(0..4).mapIt(rows.getHeight(it))
  (kind, heights)

let objects = readFile("input").strip.split("\n\n").map(parseObject)
let keys = objects.filterIt(it[0] == Key).mapIt(it[1])
let locks = objects.filterIt(it[0] == Lock).mapIt(it[1])

func fit(key, lock: seq[int]): bool =
  zip(key, lock).allIt(it[0] + it[1] <= 5)

let first = product([keys, locks]).filterIt(fit(it[0], it[1])).len
echo "First: " & $first
