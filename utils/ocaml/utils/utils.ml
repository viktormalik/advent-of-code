let rec make_set lst =
  match lst with
  | [] -> []
  | h :: t -> if List.mem h t then make_set t else h :: make_set t

let rec insert x lst =
  match lst with
  | [] -> [ [ x ] ]
  | h :: t -> (x :: lst) :: List.map (fun el -> h :: el) (insert x t)

let rec perm lst =
  match lst with
  | [] -> [ lst ]
  | h :: t -> List.flatten (List.map (insert h) (perm t))

let int_list s n = List.of_seq (Seq.take n (Seq.ints s))

let find_index pred lst =
  let rec find_index_rec i lst =
    match lst with
    | [] -> raise Not_found
    | h :: t -> if pred h then i else find_index_rec (i + 1) t
  in
  find_index_rec 0 lst

let chars_of_string str = List.init (String.length str) (String.get str)
let slice lst from n = List.filteri (fun i _ -> i >= from && i < from + n) lst

let is_sublist lst sublst =
  List.filteri (fun i _ -> slice lst i (List.length sublst) = sublst) lst != []
