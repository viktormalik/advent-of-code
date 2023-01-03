type tile = Open | Wall | Out
type inst = Turn of string | Move of int
type face = Right | Down | Left | Up
type edgedir = Horiz | Vert
type edge = { dir : edgedir; x : int; y : int }

type edgename =
  | BackBottom
  | BackRight
  | RightBottom
  | RightLeft
  | RightTop
  | TopLeft
  | TopRight
  | LeftTop
  | LeftLeft
  | FrontRight
  | FrontBottom
  | BottomBottom
  | BottomRight
  | BottomTop
;;

let map =
  let tile x = match x with '.' -> Open | '#' -> Wall | _ -> Out in
  Array.of_list
    (List.map
       (fun row -> Array.of_list (List.map tile row))
       (Utils.slice (Input.char_lists "input") 0 200))
in

let path =
  let line = List.hd (List.rev (Input.lines "input")) in
  let split = Str.full_split (Str.regexp "[LR]") line in
  let parse s =
    match s with Str.Delim d -> Turn d | Str.Text n -> Move (int_of_string n)
  in
  List.map parse split
in

(*
    BBRR
    BBRR
    TT
    TT
  LLFF
  LLFF
  BB
  BB
 *)
let edges =
  [
    (BackBottom, { dir = Horiz; x = 0; y = 50 });
    (BackRight, { dir = Vert; x = 0; y = 50 });
    (RightBottom, { dir = Horiz; x = 0; y = 100 });
    (RightLeft, { dir = Vert; x = 0; y = 149 });
    (RightTop, { dir = Horiz; x = 49; y = 100 });
    (TopLeft, { dir = Vert; x = 50; y = 50 });
    (TopRight, { dir = Vert; x = 50; y = 99 });
    (LeftTop, { dir = Horiz; x = 100; y = 0 });
    (LeftLeft, { dir = Vert; x = 100; y = 0 });
    (FrontRight, { dir = Vert; x = 100; y = 99 });
    (FrontBottom, { dir = Horiz; x = 149; y = 50 });
    (BottomBottom, { dir = Vert; x = 150; y = 0 });
    (BottomRight, { dir = Horiz; x = 199; y = 0 });
    (BottomTop, { dir = Vert; x = 150; y = 49 });
  ]
in
let opposite edge =
  match edge with
  | BackBottom -> (FrontBottom, Up, false)
  | BackRight -> (RightLeft, Left, false)
  | RightBottom -> (RightTop, Up, false)
  | RightLeft -> (BackRight, Right, false)
  | RightTop -> (RightBottom, Down, false)
  | TopLeft -> (TopRight, Left, false)
  | TopRight -> (TopLeft, Right, false)
  | LeftTop -> (BottomRight, Up, false)
  | LeftLeft -> (FrontRight, Left, false)
  | FrontRight -> (LeftLeft, Right, false)
  | FrontBottom -> (BackBottom, Down, false)
  | BottomBottom -> (BottomTop, Left, false)
  | BottomRight -> (LeftTop, Down, false)
  | BottomTop -> (BottomBottom, Right, false)
in
let cube_next edge =
  match edge with
  | BackBottom -> (BottomBottom, Right, false)
  | BackRight -> (LeftLeft, Right, true)
  | RightBottom -> (BottomRight, Up, false)
  | RightLeft -> (FrontRight, Left, true)
  | RightTop -> (TopRight, Left, false)
  | TopLeft -> (LeftTop, Down, false)
  | TopRight -> (RightTop, Up, false)
  | LeftTop -> (TopLeft, Right, false)
  | LeftLeft -> (BackRight, Right, true)
  | FrontRight -> (RightLeft, Left, true)
  | FrontBottom -> (BottomTop, Left, false)
  | BottomBottom -> (BackBottom, Down, false)
  | BottomRight -> (RightBottom, Down, false)
  | BottomTop -> (FrontBottom, Up, false)
in
let edge_pos =
  [
    (Right, [ RightLeft; TopRight; FrontRight; BottomTop ]);
    (Down, [ RightTop; FrontBottom; BottomRight ]);
    (Left, [ BackRight; TopLeft; LeftLeft; BottomBottom ]);
    (Up, [ BackBottom; RightBottom; LeftTop ]);
  ]
in

let on_edge (x, y) face (e, edge) =
  ((edge.dir = Horiz && x = edge.x && y >= edge.y && y < edge.y + 50)
  || (edge.dir = Vert && y = edge.y && x >= edge.x && x < edge.x + 50))
  && List.mem e (List.assoc face edge_pos)
in
let edge_offset (x, y) e = if e.dir = Horiz then y - e.y else x - e.x in
let edge_pos e offset =
  if e.dir = Horiz then (e.x, e.y + offset) else (e.x + offset, e.y)
in

let rec move wrapfun path (pos, face) =
  let turn face dir =
    match face with
    | Right -> if dir = "R" then Down else Up
    | Down -> if dir = "R" then Left else Right
    | Left -> if dir = "R" then Up else Down
    | Up -> if dir = "R" then Right else Left
  in
  let next (x, y) face =
    match face with
    | Right -> (x, y + 1)
    | Down -> (x + 1, y)
    | Left -> (x, y - 1)
    | Up -> (x - 1, y)
  in
  let wrap pos (e, edge) =
    let next_edge, next_face, flip = wrapfun e in
    let offset = edge_offset pos edge in
    let offset = if flip then 49 - offset else offset in
    (edge_pos (List.assoc next_edge edges) offset, next_face)
  in
  let move_one pos face =
    let (nx, ny), new_face =
      match List.find_opt (on_edge pos face) edges with
      | Some edge -> wrap pos edge
      | None -> (next pos face, face)
    in
    if map.(nx).(ny) = Wall then (pos, face) else ((nx, ny), new_face)
  in

  match path with
  | [] -> (pos, face)
  | Turn d :: p -> move wrapfun p (pos, turn face d)
  | Move 0 :: p -> move wrapfun p (pos, face)
  | Move n :: p -> move wrapfun (Move (n - 1) :: p) (move_one pos face)
in

let password ((x, y), face) =
  let faceval face =
    match face with Right -> 0 | Down -> 1 | Left -> 2 | Up -> 3
  in
  (1000 * (x + 1)) + (4 * (y + 1)) + faceval face
in

let start = ((0, 50), Right) in
let first = password (move opposite path start) in
Printf.printf "First: %d\n" first;
let second = password (move cube_next path start) in
Printf.printf "Second: %d\n" second
