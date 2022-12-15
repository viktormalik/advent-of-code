let parse_coor str =
  let n = List.nth (String.split_on_char '=' str) 1 in
  int_of_string (Input.num_drop_suffix n)
in
let parse line =
  let split = String.split_on_char ' ' line in
  ( (parse_coor (List.nth split 2), parse_coor (List.nth split 3)),
    (parse_coor (List.nth split 8), parse_coor (List.nth split 9)) )
in

let sensors = Input.parse_lines parse "input" in

let union (a1, a2) (b1, b2) =
  if a1 <= b1 && b2 <= a2 then Some (a1, a2)
  else if b1 <= a1 && a2 <= b2 then Some (b1, b2)
  else if (a1 <= b1 && b1 <= a2) || (b1 <= a2 && a2 <= b2) then Some (a1, b2)
  else if (b1 <= a1 && a1 <= b2) || (a1 <= b2 && b2 <= a2) then Some (b1, a2)
  else None
in
let rec merge segments seg =
  match segments with
  | [] -> [ seg ]
  | s :: ss -> (
      match union s seg with None -> s :: merge ss seg | Some u -> u :: ss)
in
let merge_all segments = List.fold_left merge [] segments in

let covered row ((xs, ys), (xb, yb)) =
  let dst = abs (xs - xb) + abs (ys - yb) in
  let diff = abs (ys - row) in
  let take = dst - diff in
  if diff > dst then None else Some (xs - take, xs + take)
in
let covered_all row =
  merge_all (merge_all (List.filter_map (covered row) sensors))
in

let row = 2000000 in
let row_beacons =
  Utils.make_set
    (List.filter_map
       (fun (_, (bx, by)) -> if by = row then Some (bx, by) else None)
       sensors)
in
let first =
  List.fold_left (fun sum (x1, x2) -> sum + (x2 - x1 + 1)) 0 (covered_all row)
  - List.length row_beacons
in
Printf.printf "First: %d\n" first;

let size = 4000000 in
let cover free sensor =
  let cover_row sensor row =
    match covered row sensor with
    | Some seg -> free.(row) <- merge free.(row) seg
    | None -> ()
  in
  Array.iteri (fun i _ -> cover_row sensor i) free
in
let cover_all sensors =
  let inside (x1, x2) = (x1 >= 0 && x1 <= size) || (x2 >= 0 && x2 <= size) in
  let free = Array.make (size + 1) [] in
  List.iter (cover free) sensors;
  let free = Array.map merge_all (Array.map merge_all free) in
  Array.map (List.filter inside) free
in
let free = cover_all sensors in

let getx segs =
  let _, a2 = List.nth segs 0 in
  let b1, b2 = List.nth segs 1 in
  if a2 + 1 = b1 - 1 then a2 + 1 else b2 + 1
in
let beacon =
  Array.find_map
    (fun (y, segs) -> if segs = [] then None else Some (getx segs, y))
    (Array.mapi (fun y segs -> (y, segs)) free)
in
let bx, by = Option.get beacon in
let second = (bx * size) + by in
Printf.printf "Second: %d\n" second
