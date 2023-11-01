let split = String.split_on_char ' ' (Input.read "input") in
let target_row = int_of_string (Input.num_drop_suffix (List.nth split 16)) in
let target_col = int_of_string (Input.num_drop_suffix (List.nth split 18)) in

let next (row, col, n) =
  if row = target_row && col = target_col then None
  else
    let row, col = if row = 1 then (col + 1, 1) else (row - 1, col + 1) in
    let n = n * 252533 mod 33554393 in
    Some (n, (row, col, n))
in

let first = Seq.fold_left (fun _ n -> n) 0 (Seq.unfold next (1, 1, 20151125)) in
Printf.printf "First: %d\n" first
