let input = Input.lines "input" in

let stacks =
  let parse_crane line n =
    let cs = String.sub line (4 * (n - 1)) 3 in
    if cs = "   " then None else Some (String.sub cs 1 1)
  in
  let parse_cranes line = List.map (parse_crane line) (Utils.int_list 1 9) in
  let rec add_to_stacks cranes stacks =
    match (cranes, stacks) with
    | None :: cs, s :: ss -> s :: add_to_stacks cs ss
    | Some c :: cs, s :: ss -> (c :: s) :: add_to_stacks cs ss
    | _, _ -> []
  in
  let stack_lines = List.map parse_cranes (Utils.slice input 0 8) in
  List.fold_right add_to_stacks stack_lines (Array.to_list (Array.make 9 []))
in

let proc =
  let parse_cmd line =
    let split =
      List.filter_map int_of_string_opt (String.split_on_char ' ' line)
    in
    (List.nth split 0, List.nth split 1 - 1, List.nth split 2 - 1)
  in
  List.map parse_cmd (List.filter (String.starts_with ~prefix:"move") input)
in

let move_lifo stacks (cnt, src, dst) =
  for _ = 1 to cnt do
    let crane = List.hd stacks.(src) in
    stacks.(src) <- List.tl stacks.(src);
    stacks.(dst) <- crane :: stacks.(dst)
  done;
  stacks
in

let move_fifo stacks (cnt, src, dst) =
  let cranes = Utils.slice stacks.(src) 0 cnt in
  stacks.(src) <- Utils.slice stacks.(src) cnt (List.length stacks.(src) - cnt);
  stacks.(dst) <- cranes @ stacks.(dst);
  stacks
in

let tops stacks = String.concat "" (Array.to_list (Array.map List.hd stacks)) in

let first = tops (List.fold_left move_lifo (Array.of_list stacks) proc) in
let second = tops (List.fold_left move_fifo (Array.of_list stacks) proc) in
Printf.printf "First: %s\n" first;
Printf.printf "Second: %s\n" second
