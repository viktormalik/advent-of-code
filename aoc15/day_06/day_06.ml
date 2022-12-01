type action = On | Off | Toggle
type inst = { x1 : int; y1 : int; x2 : int; y2 : int; a : action };;

let parse line =
  let coor = {|\([0-9]+\),\([0-9]+\)|} in
  let re =
    Str.regexp ({|\(turn on\|turn off\|toggle\) |} ^ coor ^ " through " ^ coor)
  in
  let _ = Str.string_match re line 0 in
  let a =
    match Str.matched_group 1 line with
    | "turn on" -> On
    | "turn off" -> Off
    | _ -> Toggle
  in
  let x1 = int_of_string (Str.matched_group 2 line) in
  let y1 = int_of_string (Str.matched_group 3 line) in
  let x2 = int_of_string (Str.matched_group 4 line) in
  let y2 = int_of_string (Str.matched_group 5 line) in
  { x1; y1; x2; y2; a }
in

let insts = Input.parse_lines parse "input" in

let elem x y = (x * 1000) + y in
let set (x, y) act lights =
  lights.(elem x y) <-
    (match act with On -> 1 | Off -> 0 | Toggle -> 1 - lights.(elem x y))
in
let adjust (x, y) act lights =
  lights.(elem x y) <-
    (match act with
    | On -> lights.(elem x y) + 1
    | Off -> max (lights.(elem x y) - 1) 0
    | Toggle -> lights.(elem x y) + 2)
in
let operate f lights inst =
  for x = inst.x1 to inst.x2 do
    for y = inst.y1 to inst.y2 do
      f (x, y) inst.a lights
    done
  done
in

let lights = Array.make (1000 * 1000) 0 in
let () = List.iter (operate set lights) insts in
let first = Array.fold_left ( + ) 0 lights in

let lights = Array.make (1000 * 1000) 0 in
let () = List.iter (operate adjust lights) insts in
let second = Array.fold_left ( + ) 0 lights in

Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
