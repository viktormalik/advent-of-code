let parse_num n = int_of_string (String.sub n 0 (String.length n - 1)) in
let parse line =
  let split = String.split_on_char ' ' line in
  let cap = parse_num (List.nth split 2) in
  let dur = parse_num (List.nth split 4) in
  let fla = parse_num (List.nth split 6) in
  let tex = parse_num (List.nth split 8) in
  let cal = int_of_string (List.nth split 10) in
  [ cap; dur; fla; tex; cal ]
in

let ings = Input.parse_lines parse "input" in

let rec next_recipe recipe =
  match recipe with
  | [ 100 ] -> None
  | 100 :: ns -> (
      match next_recipe ns with
      | Some (ns, _) -> Some (0 :: ns, 0 :: ns)
      | None -> None)
  | n :: ns -> Some ((n + 1) :: ns, (n + 1) :: ns)
  | [] -> Some ([], [])
in
let recipes =
  Seq.filter
    (fun r -> List.fold_left ( + ) 0 r == 100)
    (Seq.unfold next_recipe (List.init (List.length ings) (fun _ -> 0)))
in

let prop_ing_score recipe prop ing_i ing =
  List.nth ing prop * List.nth recipe ing_i
in
let prop_score recipe prop =
  max 0 (List.fold_left ( + ) 0 (List.mapi (prop_ing_score recipe prop) ings))
in
let score recipe =
  List.fold_left ( * ) 1 (List.map (prop_score recipe) [ 0; 1; 2; 3 ])
in

let max_score recipes = Seq.fold_left max 0 (Seq.map score recipes) in

let first = max_score recipes in
let second = max_score (Seq.filter (fun r -> prop_score r 4 == 500) recipes) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
