import std/algorithm
import std/sequtils
import std/sets
import std/strutils
import std/tables

let input = readFile("input").strip.splitLines

var lan: Table[string, HashSet[string]]

proc add(src: string, target: string) =
  if not lan.contains(src):
    lan[src] = initHashSet[string]()
  lan[src].incl(target)

for line in input:
  let split = line.split('-')
  add(split[0], split[1])
  add(split[1], split[0])

var triples: HashSet[HashSet[string]]

for computer, neighs in lan.pairs:
  if computer[0] != 't':
    continue
  for n1 in neighs:
    for n2 in neighs:
      if n1 == n2:
        continue
      if lan[n1].contains(n2):
        triples.incl(@[computer, n1, n2].toHashSet)

let first = triples.len
echo "First: " & $first

var maxClique: HashSet[string]
for computer in lan.keys:
  var clique = @[computer].toHashSet
  for c in lan.keys:
    if clique.allIt(lan[it].contains(c)):
      clique.incl(c)
  if clique.len > maxClique.len:
    maxClique = clique

let second = maxClique.items.toSeq.sorted.join(",")
echo "Second: " & $second
