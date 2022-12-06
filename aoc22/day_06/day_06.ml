let input = Input.chars "input" in

let rec marker chars prefix =
  if List.length (Utils.make_set (Utils.slice chars 0 prefix)) = prefix then
    Utils.slice chars prefix (List.length chars - prefix)
  else marker (List.tl chars) prefix
in

let first = List.length input - List.length (marker input 4) in
let second = List.length input - List.length (marker input 14) in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
