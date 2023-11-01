module Wires = Map.Make (String)

type op = IN | AND | OR | LSHIFT | RSHIFT | NOT
type rule = { in1 : string; in2 : string; out : string; op : op };;

let parse line =
  let split = Str.bounded_split (Str.regexp " -> ") line 2 in
  let left = List.nth split 0 in
  let out = List.nth split 1 in
  let re = Str.regexp {|\([a-z0-9]*\) ?\([A-Z]+\) \([a-z0-9]+\)|} in
  let m = Str.string_match re left 0 in
  let in1 = if m then Str.matched_group 1 left else left in
  let in2 = if m then Str.matched_group 3 left else "" in
  let op =
    if m then
      match Str.matched_group 2 left with
      | "AND" -> AND
      | "OR" -> OR
      | "LSHIFT" -> LSHIFT
      | "RSHIFT" -> RSHIFT
      | "NOT" -> NOT
      | _ -> IN
    else IN
  in
  { in1; in2; out; op }
in

let rules = Input.parse_lines parse "input" in

let valid wires i =
  i = "" || int_of_string_opt i != None || Wires.mem i !wires
in
let next_rule wires r =
  valid wires r.in1 && valid wires r.in2 && not (Wires.mem r.out !wires)
in
let get wires x =
  match int_of_string_opt x with Some n -> n | None -> Wires.find x !wires
in
let apply wires rule =
  match rule.op with
  | IN -> get wires rule.in1
  | AND -> get wires rule.in1 land get wires rule.in2
  | OR -> get wires rule.in1 lor get wires rule.in2
  | LSHIFT -> get wires rule.in1 lsl get wires rule.in2
  | RSHIFT -> get wires rule.in1 lsr get wires rule.in2
  | NOT -> lnot (get wires rule.in2)
in

let compute init_wires =
  let wires = ref init_wires in
  let quit = ref false in
  while not !quit do
    let rule = List.find (next_rule wires) rules in
    let out = apply wires rule in
    wires := Wires.add rule.out out !wires;
    if Wires.mem "a" !wires then quit := true
  done;
  Wires.find "a" !wires
in

let first = compute Wires.empty in
Printf.printf "First: %d\n" first;

let second = compute (Wires.singleton "b" first) in
Printf.printf "Second: %d\n" second
