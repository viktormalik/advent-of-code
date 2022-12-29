let file = Input.parse_lines int_of_string "input" in
let file = List.mapi (fun i n -> (n, i)) file in
let zero = List.find (fun (n, _) -> n = 0) file in
let cnt = List.length file in

let mix file rounds =
  let mix_one file (n, x) =
    let i = Utils.find_index (( = ) (n, x)) file in
    let new_i =
      match i + n with
      | j when j < 0 -> cnt - 1 + (j mod (cnt - 1))
      | j when j >= cnt -> j mod (cnt - 1)
      | j -> j
    in
    if new_i > i then
      Utils.slice file 0 i
      @ Utils.slice file (i + 1) (new_i - i)
      @ [ (n, x) ]
      @ Utils.slice file (new_i + 1) (cnt - new_i)
    else if i > new_i then
      Utils.slice file 0 new_i
      @ [ (n, x) ]
      @ Utils.slice file new_i (i - new_i)
      @ Utils.slice file (i + 1) (cnt - i)
    else file
  in
  let res = ref file in
  for _ = 1 to rounds do
    res := List.fold_left mix_one !res file
  done;
  !res
in

let coords file =
  let zeroi = Utils.find_index (( = ) zero) file in
  let nth offset =
    let n, _ = List.nth file ((zeroi + offset) mod cnt) in
    n
  in
  nth 1000 + nth 2000 + nth 3000
in

let first = coords (mix file 1) in
Printf.printf "First: %d\n" first;

let file = List.map (fun (n, x) -> (n * 811589153, x)) file in
let second = coords (mix file 10) in
Printf.printf "Second: %d\n" second
