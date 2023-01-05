let map = Input.matrix "input" in
let height = Array.length map in
let width = Array.length map.(0) in

let get_open row =
  let y, _ =
    Option.get
      (Array.find_opt
         (fun (_, x) -> x = '.')
         (Array.mapi (fun i x -> (i, x)) map.(row)))
  in
  y
in
let start = (0, get_open 0) in
let target = (height - 1, get_open (height - 1)) in

let blizzards =
  let blizzards = ref [] in
  for x = 0 to Array.length map - 1 do
    for y = 0 to Array.length map.(x) - 1 do
      if map.(x).(y) != '.' && map.(x).(y) != '#' then
        let dx, dy =
          match map.(x).(y) with
          | '>' -> (0, 1)
          | '<' -> (0, -1)
          | 'v' -> (1, 0)
          | '^' -> (-1, 0)
          | _ -> (0, 0)
        in
        blizzards := ((x, y), (dx, dy)) :: !blizzards
    done
  done;
  !blizzards
in
let valley =
  let cnt pos = match pos with '#' -> -1 | '.' -> 0 | _ -> 1 in
  Array.map (Array.map cnt) map
in

let move valley blizzards =
  let wrap (x, y) =
    let x = if x = 0 then height - 2 else if x = height - 1 then 1 else x in
    let y = if y = 0 then width - 2 else if y = width - 1 then 1 else y in
    (x, y)
  in
  let move_one ((x, y), (dx, dy)) =
    let nx, ny = wrap (x + dx, y + dy) in
    valley.(x).(y) <- valley.(x).(y) - 1;
    valley.(nx).(ny) <- valley.(nx).(ny) + 1;
    ((nx, ny), (dx, dy))
  in
  List.map move_one blizzards
in

let neighs valley (x, y) =
  let valid (dx, dy) =
    x + dx >= 0 && x + dx < height && valley.(x + dx).(y + dy) = 0
  in
  List.filter_map
    (fun (dx, dy) -> if valid (dx, dy) then Some (x + dx, y + dy) else None)
    [ (0, 0); (0, 1); (0, -1); (1, 0); (-1, 0) ]
in
let shortest valley blizzards start targets =
  let valley = Array.map Array.copy valley in
  let blizzards = ref blizzards in
  let targets = ref targets in

  let states = ref [ start ] in
  let quit = ref false in
  let steps = ref 0 in
  while not !quit do
    steps := !steps + 1;
    blizzards := move valley !blizzards;
    states := Utils.make_set (List.flatten (List.map (neighs valley) !states));
    if List.exists (( = ) (List.hd !targets)) !states then
      match !targets with
      | [] | _ :: [] -> quit := true
      | t :: ts ->
          states := [ t ];
          targets := ts
  done;
  !steps
in

let first = shortest valley blizzards start [ target ] in
Printf.printf "First: %d\n" first;
let second = shortest valley blizzards start [ target; start; target ] in
Printf.printf "Second: %d\n" second
