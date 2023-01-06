let input = int_of_string (Input.read "input") in
let n = input / 2 in

let deliver presents limit =
  let houses = Array.make (n + 1) 0 in
  for elf = 1 to n do
    let h = ref elf in
    let i = ref 0 in
    while !h < n && (limit = None || !i < Option.get limit) do
      houses.(!h) <- houses.(!h) + (elf * presents);
      h := !h + elf;
      i := !i + 1
    done
  done;
  houses
in

let lowest houses =
  let house, _ =
    Option.get
      (Array.find_opt
         (fun (_, v) -> v > input)
         (Array.mapi (fun h v -> (h, v)) houses))
  in
  house
in
let first = lowest (deliver 10 None) in
let second = lowest (deliver 11 (Some 50)) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
