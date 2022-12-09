let parse line =
  let split = String.split_on_char ' ' line in
  (List.nth split 0, int_of_string (List.nth split 1))
in
let motions = Input.parse_lines parse "input" in

let step_head (hx, hy) dir =
  match dir with
  | "D" -> (hx, hy - 1)
  | "U" -> (hx, hy + 1)
  | "L" -> (hx - 1, hy)
  | "R" -> (hx + 1, hy)
  | _ -> (hx, hy)
in
let follow (hx, hy) (tx, ty) =
  let dx, dy = (hx - tx, hy - ty) in
  if abs dx > 1 || abs dy > 1 then
    let tx = if dx > 0 then tx + 1 else if dx < 0 then tx - 1 else tx in
    let ty = if dy > 0 then ty + 1 else if dy < 0 then ty - 1 else ty in
    (tx, ty)
  else (tx, ty)
in
let rec follow_lst hd lst =
  match lst with
  | [] -> []
  | h :: t ->
      let nh = follow hd h in
      nh :: follow_lst nh t
in

let last ls = List.fold_left (fun _ x -> x) (List.hd ls) ls in

let rec motion knots (dir, n) =
  if n = 0 then (knots, [ last knots ])
  else
    let nh = step_head (List.hd knots) dir in
    let nt = follow_lst nh (List.tl knots) in
    let nknots, hist = motion (nh :: nt) (dir, n - 1) in
    (nknots, last knots :: hist)
in

let visited knots =
  let _, hist = List.fold_left_map motion knots motions in
  List.length (Utils.make_set (List.flatten hist))
in

let first = visited [ (0, 0); (0, 0) ] in
let second = visited (Array.to_list (Array.make 10 (0, 0))) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
