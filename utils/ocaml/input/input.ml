let read filename =
  String.trim (In_channel.with_open_text filename In_channel.input_all)

let line_to_chars line = List.init (String.length line) (String.get line)
let chars filename = line_to_chars (read filename)

let lines filename =
  let rec read_line ch =
    match In_channel.input_line ch with
    | Some line -> line :: read_line ch
    | None -> []
  in
  In_channel.with_open_text filename read_line

let char_lists filename = List.map line_to_chars (lines filename)
let parse_lines f filename = List.map f (lines filename)
