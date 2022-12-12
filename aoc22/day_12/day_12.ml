let parse line = Array.of_list (Utils.chars_of_string line) in
let map = Array.of_list (Input.parse_lines parse "input") in

let valid map dists (sx, sy) (dx, dy) =
  dx >= 0
  && dx < Array.length map
  && dy >= 0
  && dy < Array.length map.(0)
  && Char.code map.(dx).(dy) - Char.code map.(sx).(sy) <= 1
  && dists.(dx).(dy) > dists.(sx).(sy) + 1
in
let move map dists (x, y) =
  let l = (x - 1, y) in
  let r = (x + 1, y) in
  let u = (x, y - 1) in
  let d = (x, y + 1) in
  List.filter (valid map dists (x, y)) [ l; r; u; d ]
in
let dims map = (Array.length map, Array.length map.(0)) in
let shortest map start dest =
  let dimx, dimy = dims map in
  let dists = Array.make_matrix dimx dimy (dimx * dimy) in
  let sx, sy = start in
  dists.(sx).(sy) <- 0;

  let to_visit = ref [ start ] in
  let res = ref None in
  while !res = None && List.length !to_visit > 0 do
    let x, y = List.hd !to_visit in

    let next = move map dists (x, y) in
    List.iter (fun (nx, ny) -> dists.(nx).(ny) <- dists.(x).(y) + 1) next;
    to_visit :=
      List.sort
        (fun (ax, ay) (bx, by) -> compare dists.(ax).(ay) dists.(bx).(by))
        (next @ List.tl !to_visit);

    if (x, y) = dest then res := Some dists.(x).(y)
  done;
  !res
in

let find map c =
  let rec find_rec x y c =
    if map.(x).(y) = c then (x, y)
    else if y = Array.length map.(x) - 1 then find_rec (x + 1) 0 c
    else find_rec x (y + 1) c
  in
  find_rec 0 0 c
in
let mark_start_dest map (sx, sy) (dx, dy) =
  map.(sx).(sy) <- 'a';
  map.(dx).(dy) <- 'z'
in

let start = find map 'S' in
let dest = find map 'E' in
let () = mark_start_dest map start dest in

let first = shortest map start dest in
Printf.printf "First: %d\n" (Option.get first);

let all_points map =
  List.flatten
    (Array.to_list
       (Array.mapi
          (fun x row -> Array.to_list (Array.mapi (fun y c -> (x, y, c)) row))
          map))
in
let cands =
  List.filter_map
    (fun (x, y, c) -> if c = 'a' then shortest map (x, y) dest else None)
    (all_points map)
in
let second = List.fold_left min (List.nth cands 0) cands in
Printf.printf "Second: %d\n" second
