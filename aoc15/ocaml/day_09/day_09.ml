let parse line =
  let re = Str.regexp {|\(.+\) to \(.+\) = \([0-9]+\)|} in
  let _ = Str.string_match re line 0 in
  let from = Str.matched_group 1 line in
  let dest = Str.matched_group 2 line in
  let dst = int_of_string (Str.matched_group 3 line) in
  [ ((from, dest), dst); ((dest, from), dst) ]
in
let dsts = List.flatten (Input.parse_lines parse "input") in
let cities = Utils.make_set (List.map (fun ((f, _), _) -> f) dsts) in

let rec dst path =
  match path with
  | [] | [ _ ] -> 0
  | f :: t :: ls -> List.assoc (f, t) dsts + dst (t :: ls)
in
let paths = List.map dst (Utils.perm cities) in

let first = List.fold_left min (List.hd paths) paths in
let second = List.fold_left max (List.hd paths) paths in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
