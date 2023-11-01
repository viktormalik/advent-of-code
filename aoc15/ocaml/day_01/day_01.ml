let input = Input.chars "input" in

let next floor n =
  let next_floor =
    match n with '(' -> floor + 1 | ')' -> floor - 1 | _ -> floor
  in
  (next_floor, next_floor)
in
let rec findindex pred lst =
  match lst with
  | [] -> -1
  | h :: t -> if pred h then 1 else 1 + findindex pred t
in
let first, floors = List.fold_left_map next 0 input in
let second = findindex (fun x -> x < 0) floors in

Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
