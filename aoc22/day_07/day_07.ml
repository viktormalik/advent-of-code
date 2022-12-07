type dir = { dirs : string list; files : int list; parent : string }

module FS = Map.Make (String);;

let lines = Input.lines "input" in

let parse_line (cwd, fs) line =
  let split = String.split_on_char ' ' line in
  if String.starts_with ~prefix:"$ cd" line then
    match List.nth split 2 with
    | "/" -> ("/", fs)
    | ".." -> ((FS.find cwd fs).parent, fs)
    | dir -> (cwd ^ dir ^ "/", fs)
  else if String.starts_with ~prefix:"dir" line then
    let dir = cwd ^ List.nth split 1 ^ "/" in
    let add_subdir d =
      match d with
      | Some d ->
          Some { dirs = dir :: d.dirs; files = d.files; parent = d.parent }
      | None -> None
    in
    let new_fs = FS.update cwd add_subdir fs in
    let new_fs = FS.add dir { dirs = []; files = []; parent = cwd } new_fs in
    (cwd, new_fs)
  else if String.starts_with ~prefix:"$ ls" line then (cwd, fs)
  else
    let f = int_of_string (List.nth split 0) in
    let add_file d =
      match d with
      | Some d ->
          Some { dirs = d.dirs; files = f :: d.files; parent = d.parent }
      | None -> None
    in
    (cwd, FS.update cwd add_file fs)
in

let _, fs =
  List.fold_left parse_line
    ("", FS.singleton "/" { dirs = []; files = []; parent = "" })
    lines
in

let rec size dir =
  let files_sz = List.fold_left ( + ) 0 dir.files in
  let dirs = List.map (fun d -> FS.find d fs) dir.dirs in
  let dirs_sz = List.fold_left ( + ) 0 (List.map size dirs) in
  files_sz + dirs_sz
in
let size_lim lim _ dir =
  let s = size dir in
  if lim s then Some s else None
in

let first =
  FS.fold (fun _ d s -> d + s) (FS.filter_map (size_lim (( >= ) 100000)) fs) 0
in
Printf.printf "First: %d\n" first;

let required = 30000000 - (70000000 - size (FS.find "/" fs)) in
let can_delete = FS.filter_map (size_lim (( <= ) required)) fs in
let second = FS.fold (fun _ d s -> min d s) can_delete 70000000 in
Printf.printf "Second: %d\n" second
