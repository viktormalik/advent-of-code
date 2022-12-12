let insts = Input.lines "input" in

let exec n inst =
  if String.starts_with ~prefix:"addx" inst then
    let inc = int_of_string (List.nth (String.split_on_char ' ' inst) 1) in
    (n + inc, [ n; n + inc ])
  else (n, [ n ])
in

let _, cycles = List.fold_left_map exec 1 insts in
let cycles = 1 :: List.flatten cycles in

let signals =
  List.filteri
    (fun i _ -> List.mem (i + 1) [ 20; 60; 100; 140; 180; 220 ])
    (List.mapi (fun i n -> (i + 1) * n) cycles)
in
let first = List.fold_left ( + ) 0 signals in
Printf.printf "First: %d\n" first;

let draw cycles i =
  let c = List.nth cycles i in
  let px = i mod 40 in
  if c - 1 <= px && px <= c + 1 then '#' else '.'
in
Printf.printf "Second:\n";
List.iteri
  (fun i px ->
    Printf.printf "%c" px;
    if (i + 1) mod 40 = 0 then Printf.printf "\n")
  (List.map (draw cycles) (Utils.int_list 0 240))
