type pos = { x : int; y : int }

module Grid = Map.Make (struct
  type t = pos

  let compare = compare
end)
;;

let input = Input.chars "input" in

let insert_or_increase v =
  match v with Some x -> Some (x + 1) | None -> Some 1
in
let move (grid, pos) dir =
  let next =
    match dir with
    | '^' -> { x = pos.x; y = pos.y + 1 }
    | 'v' -> { x = pos.x; y = pos.y - 1 }
    | '>' -> { x = pos.x + 1; y = pos.y }
    | '<' -> { x = pos.x - 1; y = pos.y }
    | _ -> pos
  in
  (Grid.update next insert_or_increase grid, next)
in
let visited dirs =
  let houses = Grid.empty in
  let pos = { x = 0; y = 0 } in
  let houses = Grid.add pos 1 houses in
  let visits, _ = List.fold_left move (houses, pos) dirs in
  visits
in

let first = Grid.cardinal (visited input) in
Printf.printf "First: %d\n" first;

let split lst = List.fold_right (fun x (l, r) -> (x :: r, l)) lst ([], []) in
let santa, robot = split input in
let merge _ val1 val2 =
  match (val1, val2) with
  | Some x, Some y -> Some (x + y)
  | None, x | x, None -> x
in
let final_grid = Grid.merge merge (visited santa) (visited robot) in

let second = Grid.cardinal final_grid in
Printf.printf "Second: %d\n" second
