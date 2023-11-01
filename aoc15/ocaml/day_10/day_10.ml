let input =
  List.map (fun c -> Char.code c - Char.code '0') (Input.chars "input")
in

let rec generate lst num cnt res =
  match lst with
  | [] -> num :: cnt :: res
  | x :: rs ->
      if x = num then generate rs num (cnt + 1) res
      else generate rs x 1 (num :: cnt :: res)
in

let apply input n =
  let digits = ref input in
  for _ = 1 to n do
    digits := List.rev (generate !digits (List.nth !digits 0) 0 [])
  done;
  List.length !digits
in

let first = apply input 40 in
Printf.printf "First: %d\n" first;

let second = apply input 50 in
Printf.printf "Second: %d\n" second
