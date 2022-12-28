type res = { ore : int; clay : int; obs : int; geode : int }

type blueprint = {
  ore_cost : res;
  clay_cost : res;
  obs_cost : res;
  geode_cost : res;
}
;;

let parse line =
  let split = String.split_on_char ' ' line in
  let nums =
    List.map
      (fun e -> int_of_string (List.nth split e))
      [ 6; 12; 18; 21; 27; 30 ]
  in

  let e : res = { ore = 0; clay = 0; obs = 0; geode = 0 } in
  let ore_cost = { e with ore = List.nth nums 0 } in
  let clay_cost = { e with ore = List.nth nums 1 } in
  let obs_cost = { e with ore = List.nth nums 2; clay = List.nth nums 3 } in
  let geode_cost = { e with ore = List.nth nums 4; obs = List.nth nums 5 } in
  { ore_cost; clay_cost; obs_cost; geode_cost }
in
let blueprints = Input.parse_lines parse "input" in

let diff (r1 : res) (r2 : res) =
  {
    ore = r1.ore - r2.ore;
    clay = r1.clay - r2.clay;
    obs = r1.obs - r2.obs;
    geode = r1.geode - r2.geode;
  }
in
let add (r1 : res) (r2 : res) =
  {
    ore = r1.ore + r2.ore;
    clay = r1.clay + r2.clay;
    obs = r1.obs + r2.obs;
    geode = r1.geode + r2.geode;
  }
in
let geq (r1 : res) (r2 : res) =
  r1.ore >= r2.ore && r1.clay >= r2.clay && r1.obs >= r2.obs
  && r1.geode >= r2.geode
in

let geodes minutes max_limits bp =
  let build_robots time lim (res, robots) =
    if geq res bp.geode_cost then
      [ (diff res bp.geode_cost, { robots with geode = robots.geode + 1 }) ]
    else if geq res bp.clay_cost && geq res bp.obs_cost && time <= lim.clay then
      [
        (diff res bp.clay_cost, { robots with clay = robots.clay + 1 });
        (diff res bp.obs_cost, { robots with obs = robots.obs + 1 });
      ]
    else if geq res bp.ore_cost && geq res bp.obs_cost && time <= lim.ore then
      [
        (diff res bp.ore_cost, { robots with ore = robots.ore + 1 });
        (diff res bp.obs_cost, { robots with obs = robots.obs + 1 });
      ]
    else if geq res bp.obs_cost && time <= lim.obs then
      [
        (diff res bp.obs_cost, { robots with obs = robots.obs + 1 });
        (res, robots);
      ]
    else if geq res bp.clay_cost && geq res bp.ore_cost && time <= lim.ore then
      [
        (diff res bp.clay_cost, { robots with clay = robots.clay + 1 });
        (diff res bp.ore_cost, { robots with ore = robots.ore + 1 });
      ]
    else if geq res bp.clay_cost && time <= lim.clay then
      [
        (diff res bp.clay_cost, { robots with clay = robots.clay + 1 });
        (res, robots);
      ]
    else if geq res bp.ore_cost && time <= lim.ore then
      [
        (diff res bp.ore_cost, { robots with ore = robots.ore + 1 });
        (res, robots);
      ]
    else [ (res, robots) ]
  in
  let expand time lim (res, robots) =
    let new_states = build_robots time lim (res, robots) in
    List.map (fun (res, new_robots) -> (add res robots, new_robots)) new_states
  in
  let rec get_states bp lim states time =
    if List.length states > 250000 then None
    else if time = minutes then Some states
    else
      let new_states = List.flatten (List.map (expand time lim) states) in
      get_states bp lim new_states (time + 1)
  in
  let max_geode states =
    List.fold_left max 0 (List.map (fun (r, _) -> r.geode) states)
  in

  let init = { ore = 0; clay = 0; obs = 0; geode = 0 } in
  let robots = { ore = 1; clay = 0; obs = 0; geode = 0 } in
  let states = ref None in
  let lim = ref max_limits in
  let thr = max_limits.clay - 5 in
  while !states = None do
    states := get_states bp !lim [ (init, robots) ] 0;
    lim :=
      {
        !lim with
        clay = (if !lim.clay = thr then max_limits.clay else !lim.clay - 1);
        obs = (if !lim.clay = thr then !lim.obs - 1 else !lim.obs);
      }
  done;
  max_geode (Option.get !states)
in

let first_lims = { ore = 12; clay = 16; obs = 20; geode = 0 } in
let first_geodes = List.map (geodes 24 first_lims) blueprints in
let first =
  List.fold_left ( + ) 0 (List.mapi (fun i q -> (i + 1) * q) first_geodes)
in
Printf.printf "First: %d\n" first;

let second_lims = { ore = 16; clay = 22; obs = 27; geode = 0 } in
let second_geodes =
  List.map (geodes 32 second_lims) (Utils.slice blueprints 0 3)
in
let second = List.fold_left ( * ) 1 second_geodes in
Printf.printf "Second: %d\n" second
