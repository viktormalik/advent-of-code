type box = { l : int; w : int; h : int };;

let parse line =
  match List.map int_of_string (Str.split (Str.regexp "x") line) with
  | l :: w :: h :: _ -> { l; w; h }
  | _ -> { l = 0; w = 0; h = 0 }
in
let boxes = Input.parse_lines parse "input" in

let area box =
  let x = box.l * box.w in
  let y = box.l * box.h in
  let z = box.w * box.h in
  (2 * x) + (2 * y) + (2 * z) + min (min x y) z
in
let first = List.fold_left ( + ) 0 (List.map area boxes) in

let volume box = box.l * box.w * box.h in
let ribbon box =
  let x = (2 * box.l) + (2 * box.w) in
  let y = (2 * box.l) + (2 * box.h) in
  let z = (2 * box.w) + (2 * box.h) in
  min (min x y) z + volume box
in
let second = List.fold_left ( + ) 0 (List.map ribbon boxes) in

Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
