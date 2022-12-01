let input = Input.lines "input" in
let parse_cals lines =
  let rec get_cals cals lines =
    match lines with
    | [] -> []
    | "" :: t -> cals :: get_cals 0 t
    | h :: t -> get_cals (cals + int_of_string h) t
  in
  get_cals 0 lines
in
let calories = List.rev (List.sort compare (parse_cals input)) in
let first = List.hd calories in
let second = List.fold_left ( + ) 0 (Utils.slice calories 0 3) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
