type character = { hp : int; dmg : int; arm : int }
type item = { cost : int; dmg : int; arm : int };;

let tailnum line =
  let split = String.split_on_char ' ' line in
  int_of_string (List.nth split (List.length split - 1))
in
let parse_boss lines =
  let hp = tailnum (List.nth lines 0) in
  let dmg = tailnum (List.nth lines 1) in
  let arm = tailnum (List.nth lines 2) in
  { hp; dmg; arm }
in
let boss = parse_boss (Input.lines "input") in

let parse_item line =
  let split = Str.split (Str.regexp "[  ]+") line in
  let split = List.filter (fun s -> String.get s 0 <> '+') split in
  let stats = List.filter_map int_of_string_opt split in
  { cost = List.nth stats 0; dmg = List.nth stats 1; arm = List.nth stats 2 }
in
let items = Input.lines "items" in
let weapons = List.map parse_item (Utils.slice items 1 5) in
let armor = List.map parse_item (Utils.slice items 8 5) in
let rings = List.map parse_item (Utils.slice items 15 6) in

let weapon_setups = List.map (fun w -> [ w ]) weapons in
let armor_setups = [] :: List.map (fun a -> [ a ]) armor in
let ring_setups =
  let single = List.map (fun r -> [ r ]) rings in
  let double =
    List.filter
      (fun rs -> List.nth rs 0 <> List.nth rs 1)
      (List.flatten
         (List.map (fun r1 -> List.map (fun r2 -> [ r1; r2 ]) rings) rings))
  in
  [] :: (single @ double)
in

let setups =
  List.flatten
    (List.map
       (fun w ->
         List.flatten
           (List.map
              (fun a -> List.map (fun r -> w @ a @ r) ring_setups)
              armor_setups))
       weapon_setups)
in
let cost setup = List.fold_left ( + ) 0 (List.map (fun i -> i.cost) setup) in
let player setup =
  let dmg = List.fold_left ( + ) 0 (List.map (fun i -> i.dmg) setup) in
  let arm = List.fold_left ( + ) 0 (List.map (fun i -> i.arm) setup) in
  { hp = 100; dmg; arm }
in

let deal (attacker : character) (defender : character) =
  let dmg = max 1 (attacker.dmg - defender.arm) in
  (attacker, { hp = defender.hp - dmg; dmg = defender.dmg; arm = defender.arm })
in
let rec fight boss player player_turn =
  if boss.hp <= 0 then true
  else if player.hp <= 0 then false
  else if player_turn then
    let player, boss = deal player boss in
    fight boss player false
  else
    let boss, player = deal boss player in
    fight boss player true
in

let win_costs =
  List.map (fun s -> (fight boss (player s) true, cost s)) setups
in
let first =
  List.fold_left min 1000
    (List.filter_map (fun (w, c) -> if w then Some c else None) win_costs)
in
Printf.printf "First: %d\n" first;
let second =
  List.fold_left max 0
    (List.filter_map (fun (w, c) -> if w then None else Some c) win_costs)
in
Printf.printf "Second: %d\n" second
