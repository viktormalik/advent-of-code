val read : string -> string
val chars : string -> char list
val lines : string -> string list
val char_lists : string -> char list list
val parse_lines : (string -> 'a) -> string -> 'a list
val matrix : string -> char array array
val parse_pair : char -> (string -> 'a) -> string -> 'a * 'a
val num_drop_suffix : string -> string
