let parse line =
  Input.parse_pair ',' (Input.parse_pair '-' int_of_string) line
in
let pairs = Input.parse_lines parse "input" in

let cover ((l1, u1), (l2, u2)) =
  (l1 <= l2 && u2 <= u1) || (l2 <= l1 && u1 <= u2)
in
let overlap ((l1, u1), (l2, u2)) =
  (l1 <= l2 && l2 <= u1) || (l2 <= l1 && l1 <= u2)
in

let first = List.length (List.filter cover pairs) in
let second = List.length (List.filter overlap pairs) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
