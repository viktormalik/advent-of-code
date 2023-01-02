type monkey = Num of int | Op of string * string * string

module Monkeys = Map.Make (String);;

let parse line =
  let name = String.sub line 0 4 in
  let split = List.tl (String.split_on_char ' ' line) in
  let monkey =
    if List.length split = 1 then Num (int_of_string (List.hd split))
    else Op (List.nth split 0, List.nth split 1, List.nth split 2)
  in
  (name, monkey)
in

let monkeys = Monkeys.of_seq (List.to_seq (Input.parse_lines parse "input")) in

let deps =
  let rec get_deps dep =
    let ds =
      List.of_seq
        (Seq.filter_map
           (fun (n, m) ->
             match m with
             | Op (x, _, _) when x = dep -> Some n
             | Op (_, _, x) when x = dep -> Some n
             | _ -> None)
           (Monkeys.to_seq monkeys))
    in
    ds @ List.flatten (List.map get_deps ds)
  in
  "humn" :: get_deps "humn"
in

let rec yell m =
  let monkey = Monkeys.find m monkeys in
  match monkey with
  | Num n -> n
  | Op (o1, op, o2) -> (
      match op with
      | "+" -> yell o1 + yell o2
      | "-" -> yell o1 - yell o2
      | "*" -> yell o1 * yell o2
      | _ -> yell o1 / yell o2)
in

let first = yell "root" in
Printf.printf "First: %d\n" first;

let rec find m res =
  if m = "humn" then res
  else
    let monkey = Monkeys.find m monkeys in
    match monkey with
    | Num _ -> raise (Invalid_argument m)
    | Op (o1, op, o2) -> (
        let dep = if List.mem o1 deps then o1 else o2 in
        let oth = if List.mem o1 deps then o2 else o1 in
        match op with
        | "+" -> find dep (res - yell oth)
        | "-" ->
            if o1 = dep then find dep (res + yell oth)
            else find dep (yell oth - res)
        | "*" -> find dep (res / yell oth)
        | _ ->
            if o1 = dep then find dep (res * yell oth)
            else find dep (yell oth / res))
in

let dep, res =
  match Monkeys.find "root" monkeys with
  | Op (o1, _, o2) -> if List.mem o1 deps then (o1, yell o2) else (o2, yell o1)
  | Num _ -> raise (Invalid_argument "root")
in

let second = find dep res in
Printf.printf "Second: %d\n" second
