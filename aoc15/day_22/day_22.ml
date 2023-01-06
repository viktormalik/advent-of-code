type character = { hp : int; dmg : int; arm : int; mana : int }

type effect = {
  cost : int;
  timer : int;
  hp : int;
  dmg : int;
  arm : int;
  mana : int;
}
;;

let tailnum line =
  let split = String.split_on_char ' ' line in
  int_of_string (List.nth split (List.length split - 1))
in
let parse_boss lines =
  let hp = tailnum (List.nth lines 0) in
  let dmg = tailnum (List.nth lines 1) in
  { hp; dmg; arm = 0; mana = 0 }
in
let boss = parse_boss (Input.lines "input") in
let player = { hp = 50; dmg = 0; arm = 0; mana = 500 } in
let cost_limit = 1500 in

let apply effects player boss =
  let apply_one ((p, b) : character * character) e =
    if e.timer > 0 then
      let p =
        { p with hp = p.hp + e.hp; arm = p.arm + e.arm; mana = p.mana + e.mana }
      in
      let b = { b with hp = b.hp - e.dmg } in
      (p, b)
    else (p, b)
  in
  let update_timers effects =
    List.map (fun e -> { e with timer = e.timer - 1 }) effects
  in
  let player, boss = List.fold_left apply_one (player, boss) effects in
  let effects = update_timers effects in
  (effects, player, boss)
in
let clear_temp_effects effects player =
  let clear_one (p : character) e =
    if e.timer >= 0 then { p with arm = p.arm - e.arm } else p
  in
  List.fold_left clear_one player effects
in

let spells =
  [
    { cost = 53; timer = 1; hp = 0; dmg = 4; arm = 0; mana = 0 };
    { cost = 73; timer = 1; hp = 2; dmg = 2; arm = 0; mana = 0 };
    { cost = 113; timer = 6; hp = 0; dmg = 0; arm = 7; mana = 0 };
    { cost = 173; timer = 6; hp = 0; dmg = 3; arm = 0; mana = 0 };
    { cost = 229; timer = 5; hp = 0; dmg = 0; arm = 0; mana = 101 };
  ]
in

let get_min results = List.fold_left min cost_limit results in

let rec round boss (player : character) hard effects player_turn next =
  let cost = List.fold_left ( + ) 0 (List.map (fun e -> e.cost) effects) in
  let lost (player : character) effects =
    cost > cost_limit || player.hp <= 0
    || player_turn
       && (next.cost > player.mana
          || List.exists (fun e -> e.timer > 0 && e.cost = next.cost) effects)
  in

  (* hard difficulty -> loose a life *)
  let player =
    if hard && player_turn then { player with hp = player.hp - 1 } else player
  in
  let defeat = player.hp <= 0 in

  (* apply effects *)
  let effects, player, boss = apply effects player boss in
  let win = boss.hp <= 0 in
  (* boss attack *)
  let player =
    if player_turn then player
    else
      let dmg = max 1 (boss.dmg - player.arm) in
      { player with hp = player.hp - dmg }
  in
  let defeat = lost player effects || defeat in
  (* clear temporary effects *)
  let player = clear_temp_effects effects player in

  (* cast new spell *)
  let effects, player, boss =
    if player_turn then
      let player = { player with mana = player.mana - next.cost } in
      if next.timer > 1 then (next :: effects, player, boss)
      else
        let next, player, boss = apply [ next ] player boss in
        (List.hd next :: effects, player, boss)
    else (effects, player, boss)
  in

  (* next round *)
  if win then Some cost
  else if defeat then None
  else if player_turn then
    match List.filter_map (round boss player hard effects false) spells with
    | [] -> None
    | results -> Some (get_min results)
  else round boss player hard effects true next
in
let fight player boss hard =
  get_min (List.filter_map (round boss player hard [] true) spells)
in

let first = fight player boss false in
Printf.printf "First: %d\n" first;
let second = fight player boss true in
Printf.printf "Second: %d\n" second
