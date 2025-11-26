import std/algorithm
import std/math
import std/sequtils
import std/sets
import std/strutils
import std/tables

type Gate = object
  inputs: seq[string]
  output: string
  op: string

func parseValue(line: string): (string, bool) =
  let split = line.split
  (split[0][0..^2], cast[bool](parseInt(split[1])))

func parseGate(line: string): (string, Gate) =
  let split = line.split
  (split[4], Gate(inputs: @[split[0], split[2]], output: split[4], op: split[1]))

let input = readFile("input").strip.split("\n\n")
let values = input[0].splitLines.map(parseValue).toTable
var gates = input[1].splitLines.map(parseGate).toTable
let z = gates.keys.toSeq.filterIt(it[0] == 'z')

func number(values: Table[string, bool], wires: seq[string]): int =
  for w in wires:
    if values[w]:
      let n = w[1..^1].parseInt
      result += 2 ^ n

func eval(values: Table[string, bool], gate: Gate): bool =
  case gate.op:
    of "AND": return values[gate.inputs[0]] and values[gate.inputs[1]]
    of "OR": return values[gate.inputs[0]] or values[gate.inputs[1]]
    of "XOR": return values[gate.inputs[0]] xor values[gate.inputs[1]]

proc simulate(gates: Table[string, Gate], initValues: Table[string, bool]): int =
  var values = initValues
  while not z.allIt(values.contains(it)):
    for gate in gates.values:
      if gate.inputs.allIt(values.contains(it)):
        values[gate.output] = values.eval(gate)
  values.number(z)

echo "First: " & $gates.simulate(values)

func find(gates: Table[string, Gate], inputs: array[2, string], op: string): Gate =
  for g in gates.values:
    if g.inputs.toHashSet == inputs.toHashSet and g.op == op:
      return g

let bits = z.mapIt(it[1..^1].parseInt).max + 1
var carry = ""
var second: seq[string]

proc swap(a, b: string) =
  gates[a].output = b
  gates[b].output = a
  second &= @[a, b]

for b in 0 .. bits - 2:
  let xbit = "x" & b.intToStr(2)
  let ybit = "y" & b.intToStr(2)
  let zbit = "z" & b.intToStr(2)

  var res = ""
  if carry == "":
    res = gates.find([xbit, ybit], "XOR").output
    carry = gates.find([xbit, ybit], "AND").output
  else:
    var bitres = gates.find([xbit, ybit], "XOR").output
    var newres = gates.find([bitres, carry], "XOR").output
    if newres != zbit:
      if newres == "":
        let i = if gates[zbit].inputs[0] == carry: 1 else: 0
        let newbitres = gates[zbit].inputs[i]
        swap(bitres, newbitres)
        bitres = newbitres
      else:
        swap(zbit, newres)
        newres = zbit
    res = newres

    let bitcarry = gates.find([xbit, ybit], "AND").output
    carry = gates.find([bitres, carry], "AND").output
    carry = gates.find([carry, bitcarry], "OR").output

echo "Second: " & second.sorted.join(",")
