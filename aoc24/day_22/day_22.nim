import std/algorithm
import std/sequtils
import std/sets
import std/strutils
import std/tables

type
  Buyer = object
    secret: uint64
    changes: Table[(int, int, int, int), uint64]

let input = readFile("input").strip.splitLines.mapIt(cast[uint64](parseInt(it)))

func mix(secret: uint64, value: uint64): uint64 = secret xor value

func prune(secret: uint64): uint64 = secret mod 16777216

func next(secret: uint64): uint64 =
  result = secret.mix(secret * 64).prune
  result = result.mix(result div 32).prune
  result = result.mix(result * 2048).prune

func generate(secret: uint64): Buyer =
  result.secret = secret
  var changes: seq[int]
  for i in 1..2000:
    let next = result.secret.next
    let lastDigit = result.secret mod 10
    let nextDigit = next mod 10
    changes.add(cast[int](nextDigit) - cast[int](lastDigit))
    if changes.len >= 4:
      let s = (changes[^4], changes[^3], changes[^2], changes[^1])
      if not result.changes.contains(s):
        result.changes[s] = nextDigit
    result.secret = next

func sequences(buyers: seq[Buyer]): HashSet[(int, int, int, int)] =
  for buyer in buyers:
    for s in buyer.changes.keys:
      result.incl(s)

func totalPrice(buyers: seq[Buyer], s: (int, int, int, int)): uint64 =
  buyers.mapIt(if it.changes.contains(s): it.changes[s] else: 0).foldl(a + b)

let buyers = input.map(generate)

let first = buyers.mapIt(it.secret).foldl(a + b)
echo "First: " & $first

let second = max(buyers.sequences.mapIt(buyers.totalPrice(it)))
echo "Second: " & $second
