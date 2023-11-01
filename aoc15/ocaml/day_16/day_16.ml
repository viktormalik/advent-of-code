let parse_thing t =
  let re = Str.regexp {|\(.* \|^\)\([a-z]+\): \([0-9]+\)$|} in
  let _ = Str.string_match re t 0 in
  (Str.matched_group 2 t, int_of_string (Str.matched_group 3 t))
in
let parse_sue line = List.map parse_thing (String.split_on_char ',' line) in
let sues = Input.parse_lines parse_sue "input" in

let tape = Input.parse_lines parse_thing "tape" in

let match_exact tape thing = List.exists (fun t -> t = thing) tape in
let match_range tape thing =
  let match_range_things (n1, v1) (n2, v2) =
    n1 = n2
    &&
    match n1 with
    | "cats" | "trees" -> v1 > v2
    | "pomeranians" | "goldfish" -> v1 < v2
    | _ -> v1 = v2
  in
  List.exists (match_range_things thing) tape
in

let first = Utils.find_index (List.for_all (match_exact tape)) sues in
let second = Utils.find_index (List.for_all (match_range tape)) sues in
Printf.printf "First: %d\n" (first + 1);
Printf.printf "Second: %d\n" (second + 1)
