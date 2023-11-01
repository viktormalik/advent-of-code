let input = Input.read "input" in
let hash_is_valid prefix n =
  let hash = Digest.to_hex (Digest.string (input ^ string_of_int n)) in
  String.starts_with ~prefix hash
in

let first = Seq.find (hash_is_valid "00000") (Seq.ints 1) in
let second = Seq.find (hash_is_valid "000000") (Seq.ints 1) in
Printf.printf "First: %d\n" (Option.get first);
Printf.printf "Second: %d\n" (Option.get second)
