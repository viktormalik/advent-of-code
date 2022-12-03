let input = Input.char_lists "input" in

let prio c =
  if Char.lowercase_ascii c = c then Char.code c - Char.code 'a' + 1
  else Char.code c - Char.code 'A' + 27
in
let rec common lss =
  match List.hd lss with
  | [] -> '0'
  | h :: t ->
      if List.for_all (List.mem h) lss then h else common (t :: List.tl lss)
in
let dupl_prio ruck =
  let half = List.length ruck / 2 in
  prio (common [ Utils.slice ruck 0 half; Utils.slice ruck half half ])
in
let symbol_prio rucks = prio (common rucks) in
let rec groups lines g =
  match lines with
  | [] -> [ g ]
  | h :: t -> (
      match List.length g with
      | 0 | 1 | 2 -> groups t (h :: g)
      | _ -> g :: groups t [ h ])
in

let first = List.fold_left ( + ) 0 (List.map dupl_prio input) in
let second = List.fold_left ( + ) 0 (List.map symbol_prio (groups input [])) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
