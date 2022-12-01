let parse line =
  let split = String.split_on_char ' ' line in
  let n1 = List.nth split 0 in
  let n2 = List.nth split 10 in
  let n2 = String.sub n2 0 (String.length n2 - 1) in
  let u = int_of_string (List.nth split 3) in
  let happ = if List.nth split 2 = "gain" then u else -u in
  ((n1, n2), happ)
in
let rules = Input.parse_lines parse "input" in
let people = Utils.make_set (List.map (fun ((f, _), _) -> f) rules) in

let happ_list rules seat =
  let happ_pair rules n1 n2 =
    List.assoc (n1, n2) rules + List.assoc (n2, n1) rules
  in
  let rec happ_rec first seat =
    match seat with
    | [] -> []
    | [ last ] -> [ happ_pair rules first last ]
    | n1 :: n2 :: ls -> happ_pair rules n1 n2 :: happ_rec first (n2 :: ls)
  in
  happ_rec (List.hd seat) seat
in
let happiness rules seat =
  let lst = happ_list rules seat in
  List.fold_left ( + ) 0 lst
in
let happiness_skip rules seat =
  let lst = happ_list rules seat in
  let lst_skip lst n =
    let filtered = List.filteri (fun i _ -> i != n) lst in
    List.fold_left ( + ) 0 filtered
  in
  let candidates =
    List.map (lst_skip lst) (Utils.int_list 0 (List.length lst))
  in
  List.fold_left max (List.hd candidates) candidates
in

let seatings = List.map (happiness rules) (Utils.perm people) in
let first = List.fold_left max (List.hd seatings) seatings in
Printf.printf "First: %d\n" first;

let seatings_skip = List.map (happiness_skip rules) (Utils.perm people) in
let second = List.fold_left max (List.hd seatings_skip) seatings_skip in
Printf.printf "Second: %d\n" second
