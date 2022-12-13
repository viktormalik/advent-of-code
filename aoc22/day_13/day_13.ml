type packet = Num of int | List of packet list;;

let input = Input.char_lists "input" in

let read_list chars =
  let rec to_bracket lvl elem res chars =
    match chars with
    | ']' :: cs when lvl = 0 -> (res @ [ elem ], cs)
    | ',' :: cs when lvl = 0 -> to_bracket lvl [] (res @ [ elem ]) cs
    | ']' :: cs -> to_bracket (lvl - 1) (elem @ [ ']' ]) res cs
    | '[' :: cs -> to_bracket (lvl + 1) (elem @ [ '[' ]) res cs
    | c :: cs -> to_bracket lvl (elem @ [ c ]) res cs
    | [] -> ([], [])
  in
  let ls, _ = to_bracket 0 [] [] chars in
  ls
in

let rec parse_packet chars =
  match chars with
  | [] -> List []
  | c :: cs ->
      if c >= '0' && c <= '9' then
        Num (int_of_string (String.of_seq (List.to_seq (c :: cs))))
      else List (List.map parse_packet (read_list cs))
in

let rec cmp_packets p1 p2 =
  let rec cmp_lists l1 l2 =
    match (l1, l2) with
    | [], [] -> 0
    | [], _ -> -1
    | _, [] -> 1
    | x :: xs, y :: ys ->
        let r = cmp_packets x y in
        if r != 0 then r else cmp_lists xs ys
  in
  match (p1, p2) with
  | Num n1, Num n2 -> compare n1 n2
  | List l1, List l2 -> cmp_lists l1 l2
  | Num n, List l -> cmp_lists [ Num n ] l
  | List l, Num n -> cmp_lists l [ Num n ]
in

let cmp_pair input i =
  let p1 = parse_packet (List.nth input (3 * i)) in
  let p2 = parse_packet (List.nth input ((3 * i) + 1)) in
  cmp_packets p1 p2
in
let cmps =
  List.map (cmp_pair input) (Utils.int_list 0 (List.length input / 3))
in

let first =
  List.fold_left ( + ) 0
    (List.mapi (fun i r -> if r = -1 then i + 1 else 0) cmps)
in
Printf.printf "First: %d\n" first;

let d1 = List [ List [ Num 2 ] ] in
let d2 = List [ List [ Num 6 ] ] in
let all_packets =
  d1 :: d2
  :: List.filter_map
       (fun l -> if l = [] then None else Some (parse_packet l))
       input
in
let sorted =
  List.mapi (fun i p -> (i, p)) (List.sort cmp_packets all_packets)
in

let d1_i, _ = List.find (fun (_, p) -> p = d1) sorted in
let d2_i, _ = List.find (fun (_, p) -> p = d2) sorted in
let second = (d1_i + 1) * (d2_i + 1) in
Printf.printf "Second: %d\n" second
