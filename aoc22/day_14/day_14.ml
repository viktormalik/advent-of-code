module Pt = struct
  type t = int * int

  let compare = compare
end

module Pts = Set.Make (Pt);;

let rec paths points =
  let dir a b = if a > b then -1 else 1 in
  match points with
  | [] -> Pts.empty
  | (x, y) :: [] -> Pts.singleton (x, y)
  | (x1, y1) :: (x2, y2) :: ps ->
      if x1 = x2 && y1 = y2 then paths ((x2, y2) :: ps)
      else if x1 = x2 then
        Pts.add (x1, y1) (paths ((x1, y1 + dir y1 y2) :: (x2, y2) :: ps))
      else Pts.add (x1, y2) (paths ((x1 + dir x1 x2, y1) :: (x2, y2) :: ps))
in

let parse line =
  let split = Str.split (Str.regexp " -> ") line in
  let points = List.map (Input.parse_pair ',' int_of_string) split in
  paths points
in

let input = Input.parse_lines parse "input" in
let rocks = List.fold_left Pts.union Pts.empty input in
let _, bottom = Pts.fold max (Pts.map (fun (_, y) -> (0, y)) rocks) (0, 0) in
let rocks = Pts.union rocks (paths [ (0, bottom + 2); (1000, bottom + 2) ]) in
let init = (500, 0) in

let flow fallthrough sands =
  let blocked pos = Pts.mem pos rocks || Pts.mem pos sands in
  let flow_one (x, y) =
    let cands = [ (x, y + 1); (x - 1, y + 1); (x + 1, y + 1) ] in
    let next = List.find_opt (fun p -> not (blocked p)) cands in
    Option.map (fun p -> (p, p)) next
  in
  let sand = Seq.fold_left (fun _ p -> p) init (Seq.unfold flow_one init) in
  let _, y = sand in
  if (fallthrough && y >= bottom) || sand = init then None
  else Some (sand, Pts.add sand sands)
in

let sands_first = Seq.unfold (flow true) Pts.empty in
Printf.printf "First: %d\n" (Seq.length sands_first);
let sands_second = Seq.unfold (flow false) Pts.empty in
Printf.printf "Second: %d\n" (Seq.length sands_second + 1)
