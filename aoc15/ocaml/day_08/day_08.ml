let input = Input.char_lists "input" in

let rec memsize chars =
  match chars with
  | [] -> 0
  | '"' :: rs -> memsize rs
  | '\\' :: 'x' :: _ :: _ :: rs
  | '\\' :: '\\' :: rs
  | '\\' :: '"' :: rs
  | _ :: rs ->
      1 + memsize rs
in

let rec encode chars =
  match chars with
  | [] -> []
  | x :: rs when x = '\\' || x = '"' -> '\\' :: x :: encode rs
  | x :: rs -> x :: encode rs
in
let encoded_len chars = List.length (encode chars) + 2 in

let mem = List.fold_left ( + ) 0 (List.map memsize input) in
let code = List.fold_left ( + ) 0 (List.map List.length input) in
let encoded = List.fold_left ( + ) 0 (List.map encoded_len input) in
let first = code - mem in
let second = encoded - code in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
