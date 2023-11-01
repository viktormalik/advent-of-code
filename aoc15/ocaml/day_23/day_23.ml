type inst = { op : string; reg : char; offset : int };;

let parse line =
  let split = String.split_on_char ' ' line in
  let op = List.hd split in
  let reg = (List.nth split 1).[0] in
  let offset =
    if op.[0] = 'j' then int_of_string (List.nth split (List.length split - 1))
    else 0
  in
  { op; reg; offset }
in
let insts = Input.parse_lines parse "input" in

let exec init_a =
  let ip = ref 0 in
  let a = ref init_a in
  let b = ref 0 in
  while !ip >= 0 && !ip < List.length insts do
    let inst = List.nth insts !ip in
    ip := !ip + 1;
    let reg = if inst.reg = 'a' then a else b in
    match inst.op with
    | "hlf" -> reg := !reg / 2
    | "tpl" -> reg := !reg * 3
    | "inc" -> reg := !reg + 1
    | "jmp" -> ip := !ip + inst.offset - 1
    | "jie" -> if !reg mod 2 = 0 then ip := !ip + inst.offset - 1
    | "jio" -> if !reg = 1 then ip := !ip + inst.offset - 1
    | _ -> ()
  done;
  !b
in

let first = exec 0 in
Printf.printf "First: %d\n" first;
let second = exec 1 in
Printf.printf "Second: %d\n" second
