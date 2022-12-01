type reindeer = { speed : int; fly : int; rest : int };;

let parse line =
  let split = String.split_on_char ' ' line in
  let speed = int_of_string (List.nth split 3) in
  let fly = int_of_string (List.nth split 6) in
  let rest = int_of_string (List.nth split 13) in
  { speed; fly; rest }
in
let reindeer = Input.parse_lines parse "input" in

let dst t r =
  let rtt = r.fly + r.rest in
  (t / rtt * r.fly * r.speed) + (min r.fly (t mod rtt) * r.speed)
in

let lead rs t =
  let max_dst = List.fold_left max 0 (List.map (dst t) rs) in
  List.filter (fun r -> dst t r = max_dst) rs
in

let leaders = List.flatten (List.map (lead reindeer) (Utils.int_list 1 2503)) in
let points r = List.length (List.filter (fun _r -> _r = r) leaders) in

let first = List.fold_left max 0 (List.map (dst 2503) reindeer) in
Printf.printf "First: %d\n" first;
let second = List.fold_left max 0 (List.map points reindeer) in
Printf.printf "Second: %d\n" second
