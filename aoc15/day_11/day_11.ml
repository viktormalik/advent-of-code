let input = Input.chars "input" in

let next pwd =
  let rec next_rec pwd =
    match pwd with
    | [] -> []
    | 'z' :: rs -> 'a' :: next_rec rs
    | x :: rs -> Char.chr (Char.code x + 1) :: rs
  in
  List.rev (next_rec (List.rev pwd))
in

let rec straight pwd =
  match pwd with
  | [] -> false
  | a :: b :: c :: _
    when Char.code a = Char.code b - 1 && Char.code a = Char.code c - 2 ->
      true
  | _ :: ls -> straight ls
in
let invalid_letter pwd =
  List.mem 'i' pwd || List.mem 'o' pwd || List.mem 'l' pwd
in
let pairs pwd =
  let rec pairs_cnt pwd =
    match pwd with
    | [] -> 0
    | a :: b :: ls when a = b -> pairs_cnt ls + 1
    | _ :: ls -> pairs_cnt ls
  in
  pairs_cnt pwd >= 2
in
let valid pwd = straight pwd && (not (invalid_letter pwd)) && pairs pwd in

let next_valid pwd =
  let next_pwd = ref pwd in
  let quit = ref false in
  while not !quit do
    next_pwd := next !next_pwd;
    if valid !next_pwd then quit := true
  done;
  !next_pwd
in

let pwd = next_valid input in
let first = String.concat "" (List.map (String.make 1) pwd) in
Printf.printf "First: %s\n" first;

let pwd = next_valid pwd in
let second = String.concat "" (List.map (String.make 1) pwd) in
Printf.printf "Second: %s\n" second
