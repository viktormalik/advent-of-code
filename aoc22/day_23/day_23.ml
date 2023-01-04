let init_grove =
  Array.of_list
    (List.map (fun row -> Array.of_list row) (Input.char_lists "input"))
in

let init_dirs =
  [
    [ (-1, 0); (-1, -1); (-1, 1) ];
    [ (1, 0); (1, -1); (1, 1) ];
    [ (0, -1); (-1, -1); (1, -1) ];
    [ (0, 1); (-1, 1); (1, 1) ];
  ]
in
let all_dirs =
  [ (-1, -1); (-1, 0); (-1, 1); (0, -1); (0, 1); (1, -1); (1, 0); (1, 1) ]
in

let bounds elves =
  let minx = List.fold_left min 1 (List.map (fun (x, _) -> x) elves) in
  let maxx = List.fold_left max 0 (List.map (fun (x, _) -> x) elves) in
  let miny = List.fold_left min 1 (List.map (fun (_, y) -> y) elves) in
  let maxy = List.fold_left max 0 (List.map (fun (_, y) -> y) elves) in
  (minx, maxx, miny, maxy)
in
let in_grove grove (x, y) =
  x >= 0 && x < Array.length grove && y >= 0 && y < Array.length grove.(x)
in

let elves grove =
  List.flatten
    (List.map
       (fun x ->
         List.filter_map
           (fun y -> if grove.(x).(y) = '#' then Some (x, y) else None)
           (Utils.int_list 0 (Array.length grove.(x))))
       (Utils.int_list 0 (Array.length grove)))
in
let free grove (x, y) dir =
  let free_tile (x, y) = (not (in_grove grove (x, y))) || grove.(x).(y) = '.' in
  List.for_all (fun (dx, dy) -> free_tile (x + dx, y + dy)) dir
in
let next grove dirs (x, y) =
  if free grove (x, y) all_dirs then (x, y)
  else
    match List.find_opt (free grove (x, y)) dirs with
    | Some dir ->
        let dx, dy = List.hd dir in
        (x + dx, y + dy)
    | None -> (x, y)
in

let make_proposals grove dirs elves =
  let prop_map =
    Array.make_matrix (Array.length grove) (Array.length grove.(0)) 0
  in
  let make_prop elf =
    let x, y = next grove dirs elf in
    if in_grove grove (x, y) then prop_map.(x).(y) <- prop_map.(x).(y) + 1;
    (x, y)
  in
  let props = List.map (fun e -> (e, make_prop e)) elves in
  (props, prop_map)
in

let make_grove elves =
  let minx, maxx, miny, maxy = bounds elves in
  let grove = Array.make_matrix (maxx - minx + 1) (maxy - miny + 1) '.' in
  List.iter (fun (x, y) -> grove.(x - minx).(y - miny) <- '#') elves;
  grove
in

let round grove dirs =
  let elves = elves grove in
  let proposals, prop_map = make_proposals grove dirs elves in
  let can_move (px, py) =
    (not (in_grove grove (px, py))) || prop_map.(px).(py) = 1
  in
  let new_elves =
    List.map (fun (e, p) -> if can_move p then p else e) proposals
  in
  (make_grove new_elves, new_elves <> elves)
in

let simulate rounds =
  let grove = ref init_grove in
  let dirs = ref init_dirs in
  let r = ref 0 in
  let quit = ref false in
  while not !quit do
    let g, moved = round !grove !dirs in
    grove := g;
    dirs := List.tl !dirs @ [ List.hd !dirs ];

    if Some !r = rounds || not moved then quit := true;
    r := !r + 1
  done;
  (!grove, !r)
in

let empty grove =
  Array.fold_left ( + ) 0
    (Array.map
       (fun row ->
         Array.fold_left ( + ) 0
           (Array.map (fun x -> if x = '.' then 1 else 0) row))
       grove)
in

let grove, _ = simulate (Some 10) in
let first = empty grove in
Printf.printf "First: %d\n" first;

let _, second = simulate None in
Printf.printf "Second: %d\n" second
