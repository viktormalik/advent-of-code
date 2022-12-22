type state = { room : bool list; height : int; dir : int };;

let pushes = Array.of_list (Input.chars "input") in

let shapes =
  [
    [ (0, 0); (1, 0); (2, 0); (3, 0) ];
    [ (1, 0); (0, 1); (1, 1); (2, 1); (1, 2) ];
    [ (0, 0); (1, 0); (2, 0); (2, 1); (2, 2) ];
    [ (0, 0); (0, 1); (0, 2); (0, 3) ];
    [ (0, 0); (1, 0); (0, 1); (1, 1) ];
  ]
in

let can_fall room shape =
  List.for_all (fun (x, y) -> room.(y - 1).(x) = false) shape
in
let can_move_right room shape =
  List.for_all (fun (x, _) -> x < 6) shape
  && List.for_all (fun (x, y) -> room.(y).(x + 1) = false) shape
in
let can_move_left room shape =
  List.for_all (fun (x, _) -> x > 0) shape
  && List.for_all (fun (x, y) -> room.(y).(x - 1) = false) shape
in

let down shape = List.map (fun (x, y) -> (x, y - 1)) shape in
let left room shape =
  List.map
    (fun (x, y) -> ((if can_move_left room shape then x - 1 else x), y))
    shape
in
let right room shape =
  List.map
    (fun (x, y) -> ((if can_move_right room shape then x + 1 else x), y))
    shape
in

let top room =
  let rec find_top room y =
    if Array.for_all (fun x -> x = false) room.(y) then y - 1
    else find_top room (y + 1)
  in
  find_top room 0
in
let init room shape =
  List.map (fun (x, y) -> (x + 2, y + top room + 4)) shape
in

let fall_one room dir shape =
  let shape =
    match pushes.(dir) with
    | '>' -> right room shape
    | '<' -> left room shape
    | _ -> shape
  in
  let can_fall = can_fall room shape in
  let shape = if can_fall then down shape else shape in
  (shape, not can_fall)
in

let fall room dir shape =
  let shape = ref (init room shape) in
  let dir = ref dir in
  let stop = ref false in
  while not !stop do
    let new_shape, st = fall_one room !dir !shape in
    shape := new_shape;
    dir := (!dir + 1) mod Array.length pushes;
    if st then stop := true
  done;
  List.iter (fun (x, y) -> room.(y).(x) <- true) !shape;
  !dir
in

let head room =
  let y = ref (top room) in
  let res = ref [] in
  while room.(!y) <> Array.make 7 true do
    res := !res @ Array.to_list room.(!y);
    y := !y - 1
  done;
  !res
in

let may_cache cache iter = iter mod 5 = 0 && iter < 5 * Array.length cache in

let find_period cache iter =
  if not (may_cache cache iter) then None
  else
    let last = iter / 5 in
    let rec find_index i =
      if i >= last then None
      else if
        cache.(i).room = cache.(last).room
        && cache.(i - 1).room = cache.(last - 1).room
      then Some i
      else find_index (i + 1)
    in
    find_index 1
in

let save_cache cache room dir iter =
  if may_cache cache iter then
    cache.(iter / 5) <- { room = head room; height = top room; dir }
in

let simulate n =
  let room = Array.make_matrix 6000 7 false in
  room.(0) <- Array.make 7 true;

  let shape = ref 0 in
  let dir = ref 0 in
  let i = ref 0 in
  let cache = Array.make 1000 { room = []; height = 0; dir = 0 } in
  let height_diff = ref 0 in
  while !i < n do
    let new_dir = fall room !dir (List.nth shapes !shape) in
    shape := (!shape + 1) mod List.length shapes;

    let c = !i / 5 in
    save_cache cache room !dir !i;

    match find_period cache !i with
    | Some p ->
        let period = (c - p) * 5 in
        let cycles = (n - !i) / period in
        height_diff := (cache.(c).height - cache.(p).height) * cycles;
        dir :=
          (new_dir + ((cache.(c).dir - cache.(p).dir) * cycles))
          mod Array.length pushes;
        i := n - ((n - !i) mod period) + 1
    | None ->
        i := !i + 1;
        dir := new_dir mod Array.length pushes
  done;
  top room + !height_diff
in

let first = simulate 2022 in
Printf.printf "First: %d\n" first;
let first = simulate 1000000000000 in
Printf.printf "Second: %d\n" first
