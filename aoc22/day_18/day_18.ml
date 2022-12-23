let parse line =
  let split = List.map int_of_string (String.split_on_char ',' line) in
  (List.nth split 0, List.nth split 1, List.nth split 2)
in
let cubes = Input.parse_lines parse "input" in

let xmax, ymax, zmax =
  List.fold_left
    (fun (xmax, ymax, zmax) (x, y, z) -> (max x xmax, max y ymax, max z zmax))
    (0, 0, 0) cubes
in
let xmin, ymin, zmin =
  List.fold_left
    (fun (xmin, ymin, zmin) (x, y, z) -> (min x xmin, min y ymin, min z zmin))
    (0, 0, 0) cubes
in

let valid (x, y, z) =
  x >= xmin - 1
  && x <= xmax + 1
  && y >= ymin - 1
  && y <= ymax + 1
  && z >= zmin - 1
  && z <= zmax + 1
in
let neighs (x, y, z) =
  List.filter
    (fun c -> valid c && not (List.mem c cubes))
    [
      (x - 1, y, z);
      (x + 1, y, z);
      (x, y - 1, z);
      (x, y + 1, z);
      (x, y, z - 1);
      (x, y, z + 1);
    ]
in
let outer =
  let start = (xmin - 1, ymin - 1, zmin - 1) in
  let todo = ref [ start ] in
  let res = ref [ start ] in
  while !todo <> [] do
    let cur = List.hd !todo in
    let neighs = List.filter (fun n -> not (List.mem n !res)) (neighs cur) in
    todo := List.tl !todo @ neighs;
    res := !res @ neighs
  done;
  !res
in

let adj_front (x1, y1, z1) (x2, y2, z2) = x1 = x2 && y1 = y2 && z2 - z1 = 1 in
let adj_back (x1, y1, z1) (x2, y2, z2) = x1 = x2 && y1 = y2 && z1 - z2 = 1 in
let adj_top (x1, y1, z1) (x2, y2, z2) = x1 = x2 && y2 - y1 = 1 && z1 = z2 in
let adj_bottom (x1, y1, z1) (x2, y2, z2) = x1 = x2 && y1 - y2 = 1 && z1 = z2 in
let adj_right (x1, y1, z1) (x2, y2, z2) = x2 - x1 = 1 && y1 = y2 && z1 = z2 in
let adj_left (x1, y1, z1) (x2, y2, z2) = x1 - x2 = 1 && y1 = y2 && z1 = z2 in

let is_adj cube cubes fn = List.exists (fn cube) cubes in
let adj_cnt cube cubes =
  let fns = [ adj_front; adj_back; adj_top; adj_bottom; adj_right; adj_left ] in
  List.length (List.filter (is_adj cube cubes) fns)
in

let surface cube = 6 - adj_cnt cube cubes in
let first = List.fold_left ( + ) 0 (List.map surface cubes) in
Printf.printf "First: %d\n" first;
let exterior cube = adj_cnt cube outer in
let second = List.fold_left ( + ) 0 (List.map exterior cubes) in
Printf.printf "Second: %d\n" second
