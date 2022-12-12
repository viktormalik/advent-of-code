type monkey = {
  mutable items : int list;
  op : string;
  arg1 : string;
  arg2 : string;
  test : int;
  t : int;
  f : int;
  mutable inspected : int;
}
;;

let input = Input.read "input" in

let parse_monkey monkey =
  let lines = String.split_on_char '\n' monkey in
  let words =
    List.map (fun l -> String.split_on_char ' ' (String.trim l)) lines
  in
  let items =
    let prefix_len = String.length "  Starting after: " in
    let item_str = Str.string_after (List.nth lines 1) prefix_len in
    List.map int_of_string (Str.split (Str.regexp ", ") item_str)
  in
  let get_word l w = List.nth (List.nth words l) w in
  let op = get_word 2 4 in
  let arg1 = get_word 2 3 in
  let arg2 = get_word 2 5 in
  let test = int_of_string (get_word 3 3) in
  let t = int_of_string (get_word 4 5) in
  let f = int_of_string (get_word 5 5) in
  { items; op; arg1; arg2; test; t; f; inspected = 0 }
in
let parse_monkeys input =
  Array.of_list
    (List.map parse_monkey (Str.split (Str.regexp {|Monkey [0-9]:|}) input))
in

let operation mon item globmod =
  let opval arg = if arg = "old" then item else int_of_string arg in
  let arg1 = opval mon.arg1 mod globmod in
  let arg2 = opval mon.arg2 mod globmod in
  match mon.op with
  | "+" -> arg1 + arg2
  | "*" -> arg1 * arg2
  | _ -> raise (Invalid_argument mon.op)
in
let move monkeys src dest item =
  monkeys.(src).items <- List.tl monkeys.(src).items;
  monkeys.(dest).items <- monkeys.(dest).items @ [ item ]
in
let throw monkeys relief globmod m =
  let mon = monkeys.(m) in
  let item = List.hd mon.items in
  let worry = operation mon item globmod / relief in
  let dest = if worry mod mon.test = 0 then mon.t else mon.f in
  move monkeys m dest worry;
  mon.inspected <- mon.inspected + 1
in
let turn monkeys relief globmod m =
  for _ = 0 to List.length monkeys.(m).items - 1 do
    throw monkeys relief globmod m
  done
in
let observe rounds relief =
  let monkeys = parse_monkeys input in
  let globmod = Array.fold_left ( * ) 1 (Array.map (fun m -> m.test) monkeys) in
  for _ = 1 to rounds do
    Array.iteri (fun i _ -> turn monkeys relief globmod i) monkeys
  done;
  let active = Array.map (fun m -> m.inspected) monkeys in
  Array.sort (fun x y -> -compare x y) active;
  active.(0) * active.(1)
in

let first = observe 20 3 in
Printf.printf "First: %d\n" first;
let second = observe 10000 1 in
Printf.printf "Second: %d\n" second
