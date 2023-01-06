let input = Input.lines "input" in

let parse_seq str =
  let split = Str.full_split (Str.regexp "[A-Z]") str in
  let rec merge_split s =
    match s with
    | Str.Delim d :: Str.Text t :: ss -> (d ^ t) :: merge_split ss
    | Str.Delim d :: ss -> d :: merge_split ss
    | _ -> []
  in
  merge_split split
in
let parse_rule line =
  let split = String.split_on_char ' ' line in
  (List.nth split 0, parse_seq (List.nth split 2))
in
let rules =
  List.map parse_rule (List.filter (fun l -> String.contains l '=') input)
in
let molecule = parse_seq (List.fold_left (fun _ x -> x) "" input) in

let expand_at molecule pos (_, prod) =
  let expand i x = if i = pos then prod else [ x ] in
  List.flatten (List.mapi expand molecule)
in
let expand_all_at molecule pos _ =
  let expandable (s, _) = s = List.nth molecule pos in
  let rules = List.filter expandable rules in
  List.map (expand_at molecule pos) rules
in
let expand_all molecule =
  let products = List.flatten (List.mapi (expand_all_at molecule) molecule) in
  List.filter (fun m -> m <> molecule) (Utils.make_set products)
in
let reduce_at molecule pos (src, prod) =
  let reduce i x =
    if i < pos || i >= pos + List.length prod then [ x ]
    else if i = pos then [ src ]
    else []
  in
  List.flatten (List.mapi reduce molecule)
in

let reduce_rules molecule pos =
  List.filter
    (fun (_, prod) -> Utils.slice molecule pos (List.length prod) = prod)
    rules
in
let next_rule molecule =
  let rules = List.mapi (fun i _ -> (i, reduce_rules molecule i)) molecule in
  List.find (fun (_, rs) -> rs <> []) (List.rev rules)
in
let reduce_step molecule =
  let pos, rs = next_rule molecule in
  reduce_at molecule pos (List.hd rs)
in
let reduce_rl molecule =
  let current = ref molecule in
  let i = ref 0 in
  while !current <> [ "e" ] do
    current := reduce_step !current;
    i := !i + 1
  done;
  !i
in

let first = List.length (expand_all molecule) in
Printf.printf "First: %d\n" first;
let second = reduce_rl molecule in
Printf.printf "Second: %d\n" second
