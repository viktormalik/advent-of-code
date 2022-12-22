type valve = { rate : int; dest : string list }
type act = Visit of string | Open of string;;

let parse line =
  let re =
    Str.regexp
      {|^Valve \([A-Z]+\) has flow rate=\([0-9]+\); tunnels? leads? to valves? \([A-Z, ]+\)$|}
  in
  let _ = Str.string_match re line 0 in
  let name = Str.matched_group 1 line in
  let rate = int_of_string (Str.matched_group 2 line) in
  let dest = Str.split (Str.regexp ", ") (Str.matched_group 3 line) in
  (name, { rate; dest })
in
let valves = Input.parse_lines parse "input" in
let pres_valves =
  List.filter_map
    (fun (v, valve) -> if valve.rate > 0 then Some v else None)
    valves
in

let shortest src dst =
  let next_paths p =
    List.filter_map
      (fun n -> if List.mem n p then None else Some (n :: p))
      (List.assoc (List.hd p) valves).dest
  in
  let rec find dst paths =
    match List.find_opt (fun p -> List.hd p = dst) paths with
    | Some p -> List.tl (List.rev p)
    | None ->
        let next = List.flatten (List.map next_paths paths) in
        find dst next
  in
  find dst [ [ src ] ]
in

let short_paths =
  List.map (fun v -> (("AA", v), shortest "AA" v)) pres_valves
  @ List.flatten
      (List.map
         (fun v1 -> List.map (fun v2 -> ((v1, v2), shortest v1 v2)) pres_valves)
         pres_valves)
in

let rec paths all_paths allow_revisit =
  let next_moves (path, time) =
    let cur = match List.hd path with Visit v | Open v -> v in
    let next =
      List.filter_map
        (fun d ->
          if d = cur then None else Some (d, List.assoc (cur, d) short_paths))
        pres_valves
    in
    List.filter_map
      (fun (n, p) ->
        if
          List.mem (Open n) path
          || (not allow_revisit)
             && List.exists (fun x -> List.mem (Visit x) path) p
        then None
        else if List.length p + 1 > time then
          Some (List.map (fun _ -> Visit cur) (Utils.int_list 0 time) @ path, 0)
        else
          Some
            ( (Open n :: List.map (fun v -> Visit v) (List.rev p)) @ path,
              time - List.length p - 1 ))
      next
  in

  let fin, op = List.partition (fun (_, t) -> t = 0) all_paths in
  if op = [] then fin
  else paths (List.flatten (List.map next_moves op) @ fin) allow_revisit
in

let pressure path =
  let press_act act t =
    match act with Visit _ -> 0 | Open v -> (List.assoc v valves).rate * t
  in
  List.fold_left ( + ) 0 (List.mapi (fun t n -> press_act n t) path)
in
let pressure_pair (p1, p2) = pressure p1 + pressure p2 in

let first_paths = paths [ ([ Visit "AA" ], 30) ] false in
let first =
  List.fold_left max 0 (List.map (fun (p, _) -> pressure p) first_paths)
in
Printf.printf "First: %d\n" first;

let second_paths = paths [ ([ Visit "AA" ], 26) ] true in
let second_paths =
  List.sort
    (fun (p1, _) (p2, _) -> compare (pressure p2) (pressure p1))
    second_paths
in
let all_combs =
  let compatible p1 p2 =
    let conflict act =
      match act with Open v -> List.mem (Open v) p2 | Visit _ -> false
    in
    not (List.exists conflict p1)
  in
  let res = ref [] in
  for i = 1 to 1000 do
    for j = 1 to 1500 do
      let p1, _ = List.nth second_paths i in
      let p2, _ = List.nth second_paths j in
      if compatible p1 p2 then res := (p1, p2) :: !res
    done
  done;
  !res
in
let second = List.fold_left max 0 (List.map pressure_pair all_combs) in
Printf.printf "Second: %d\n" second
