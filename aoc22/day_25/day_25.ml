let numbers = Input.char_lists "input" in

let decode snafu =
  let digit d =
    match d with '2' -> 2 | '1' -> 1 | '-' -> -1 | '=' -> -2 | _ -> 0
  in
  List.fold_left (fun res x -> (res * 5) + digit x) 0 snafu
in

let encode number =
  let rec encode_rec carry n =
    let digit, c =
      match (n mod 5) + carry with
      | 0 -> ('0', 0)
      | 1 -> ('1', 0)
      | 2 -> ('2', 0)
      | 3 -> ('=', 1)
      | 4 -> ('-', 1)
      | 5 -> ('0', 1)
      | _ -> ('1', 1)
    in
    if n / 5 = 0 then
      if c > 0 then [ Char.chr (c + Char.code '0'); digit ] else [ digit ]
    else encode_rec c (n / 5) @ [ digit ]
  in
  String.of_seq (List.to_seq (encode_rec 0 number))
in

let first = encode (List.fold_left ( + ) 0 (List.map decode numbers)) in
Printf.printf "First: %s\n" first
