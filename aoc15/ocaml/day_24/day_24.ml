let pkgs = Input.parse_lines int_of_string "input" in
let weight group = List.fold_left ( + ) 0 group in
let entanglement group = List.fold_left ( * ) 1 group in

let balance groups_cnt =
  let target = weight pkgs / groups_cnt in
  let rec power_set set remaining =
    let valid subset =
      List.length subset <= 8
      && weight subset <= target
      && weight subset + weight remaining >= target
    in
    match set with
    | [] -> [ [] ]
    | n :: ns ->
        let pns = power_set ns (n :: remaining) in
        List.filter valid (pns @ List.map (fun ss -> n :: ss) pns)
  in
  let groups = List.filter (fun g -> weight g = target) (power_set pkgs []) in
  let compare_groups g1 g2 =
    compare (List.length g1, entanglement g1) (List.length g2, entanglement g2)
  in
  let groups = List.sort compare_groups groups in
  entanglement (List.hd groups)
in

let first = balance 3 in
Printf.printf "First: %d\n" first;
let second = balance 4 in
Printf.printf "Second: %d\n" second
