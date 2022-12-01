let lines =
  List.map
    (fun l -> List.init (String.length l) (String.get l))
    (Input.lines "input")
in

let vowel c = match c with 'a' | 'e' | 'i' | 'o' | 'u' -> true | _ -> false in
let vowels line = List.length (List.filter vowel line) >= 3 in

let rec double line =
  match line with
  | [] | _ :: [] -> false
  | f :: s :: ls -> if f == s then true else double (s :: ls)
in

let rec contains (x, y) line =
  match line with
  | [] | _ :: [] -> false
  | f :: s :: ls -> if f == x && s == y then true else contains (x, y) (s :: ls)
in

let no_forbidden line =
  Bool.not
    (List.exists
       (fun pair -> contains pair line)
       [ ('a', 'b'); ('c', 'd'); ('p', 'q'); ('x', 'y') ])
in

let rec twice_double line =
  match line with
  | [] | _ :: [] -> false
  | f :: s :: ls -> if contains (f, s) ls then true else twice_double (s :: ls)
in

let rec triplet line =
  match line with
  | [] | _ :: [] | [ _; _ ] -> false
  | f :: s :: t :: ls -> if f == t then true else triplet (s :: t :: ls)
in

let is_match rules line = List.for_all (fun r -> r line) rules in

let first =
  List.length (List.filter (is_match [ vowels; double; no_forbidden ]) lines)
in
let second =
  List.length (List.filter (is_match [ twice_double; triplet ]) lines)
in
Printf.printf "First: %d\n" first;
Printf.printf "Second: %d\n" second
