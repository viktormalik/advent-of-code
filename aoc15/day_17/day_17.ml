let containers = Input.parse_lines int_of_string "input" in

let sum lst = List.fold_left ( + ) 0 lst in
let rec combs lst =
  match lst with
  | [] -> []
  | h :: t ->
      let cs = combs t in
      let res = ([ h ] :: cs) @ List.map (fun c -> h :: c) cs in
      List.filter (fun lst -> sum lst <= 150) res
in
let cs = List.filter (fun lst -> sum lst = 150) (combs containers) in

let first = List.length cs in
let min_len = List.fold_left min 150 (List.map List.length cs) in
let second = List.length (List.filter (fun l -> List.length l = min_len) cs) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
