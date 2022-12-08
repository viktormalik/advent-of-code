let trees = List.map (List.map Utils.int_of_char) (Input.char_lists "input") in

let row trees r = List.nth trees r in
let col trees c = List.map (fun row -> List.nth row c) trees in

let left trees r c = List.rev (Utils.slice (row trees r) 0 c) in
let right trees r c = Utils.slice_end (row trees r) (c + 1) in
let up trees r c = List.rev (Utils.slice (col trees c) 0 r) in
let down trees r c = Utils.slice_end (col trees c) (r + 1) in
let dirs = [ left; right; up; down ] in

let shorter ts t = ts = [] || List.for_all (( > ) t) ts in
let is_visible trees r c t =
  List.exists Fun.id (List.map (fun dir -> shorter (dir trees r c) t) dirs)
in

let visible ts t =
  let its = List.mapi (fun i x -> (i, x)) ts in
  match List.find_opt (fun (_, x) -> x >= t) its with
  | Some (i, _) -> i + 1
  | None -> List.length ts
in
let tree_score trees r c t =
  List.fold_left ( * ) 1 (List.map (fun dir -> visible (dir trees r c) t) dirs)
in

let flat_mapi f ls = List.flatten (List.mapi f ls) in
let first =
  List.length
    (flat_mapi (fun r row -> List.filteri (is_visible trees r) row) trees)
in
let second =
  List.fold_left max 0
    (flat_mapi (fun r row -> List.mapi (tree_score trees r) row) trees)
in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
