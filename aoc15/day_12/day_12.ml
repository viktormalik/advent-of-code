let input = Yojson.Basic.from_file "input" in

let is_red (_, v) =
  let open Yojson.Basic.Util in
  try to_string v = "red" with Type_error _ -> false
in

let rec sum nored json =
  match json with
  | `Int i -> i
  | `List l -> List.fold_left ( + ) 0 (List.map (sum nored) l)
  | `Assoc a ->
      if nored && List.exists is_red a then 0
      else
        List.fold_left ( + ) 0
          (List.map (sum nored) (List.map (fun (_, v) -> v) a))
  | _ -> 0
in

let first = sum false input in
let second = sum true input in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
