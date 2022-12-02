type shape = Rock | Paper | Scissors
type res = Win | Draw | Loose;;

let ps s =
  match s with "A" | "X" -> Rock | "B" | "Y" -> Paper | _ -> Scissors
in
let pr r = match r with "X" -> Loose | "Y" -> Draw | _ -> Win in
let parse_line pf ps line =
  let split = String.split_on_char ' ' line in
  (pf (List.nth split 0), ps (List.nth split 1))
in

let outcome (x, y) =
  match (x, y) with
  | Rock, Paper | Paper, Scissors | Scissors, Rock -> Win
  | Rock, Rock | Paper, Paper | Scissors, Scissors -> Draw
  | Rock, Scissors | Paper, Rock | Scissors, Paper -> Loose
in
let score_shape s = match s with Rock -> 1 | Paper -> 2 | Scissors -> 3 in
let score_res r = match r with Win -> 6 | Draw -> 3 | Loose -> 0 in
let score_round (x, y) = score_shape y + score_res (outcome (x, y)) in

let other (x, res) =
  match (x, res) with
  | Rock, Loose | Paper, Win | Scissors, Draw -> Scissors
  | Rock, Draw | Paper, Loose | Scissors, Win -> Rock
  | Rock, Win | Paper, Draw | Scissors, Loose -> Paper
in

let rounds_first = Input.parse_lines (parse_line ps ps) "input" in
let first = List.fold_left ( + ) 0 (List.map score_round rounds_first) in

let rounds_second = Input.parse_lines (parse_line ps pr) "input" in
let second =
  List.fold_left ( + ) 0
    (List.map (fun (x, y) -> score_round (x, other (x, y))) rounds_second)
in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
