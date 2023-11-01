let input = Input.char_lists "input" in
let init = Array.make_matrix 100 100 0 in
let () =
  List.iteri
    (fun x row ->
      List.iteri (fun y v -> init.(x).(y) <- (if v = '#' then 1 else 0)) row)
    input
in

let neighs x y =
  List.filter
    (fun (xn, yn) ->
      (xn != x || yn != y) && xn >= 0 && yn >= 0 && xn < 100 && yn < 100)
    (List.flatten
       (List.map
          (fun xs -> List.map (fun ys -> (x + xs, y + ys)) [ -1; 0; 1 ])
          [ -1; 0; 1 ]))
in
let on ls grid =
  List.length (List.filter (fun (x, y) -> grid.(x).(y) = 1) ls)
in

let set_corners grid =
  List.iter
    (fun (x, y) -> grid.(x).(y) <- 1)
    [ (0, 0); (0, 99); (99, 0); (99, 99) ]
in
let step grid corners_on =
  let old = Array.(map copy) grid in
  for x = 0 to 99 do
    for y = 0 to 99 do
      let cnt = on (neighs x y) old in
      grid.(x).(y) <-
        (match (old.(x).(y), cnt) with 1, 2 | 1, 3 | 0, 3 -> 1 | _, _ -> 0)
    done
  done;
  if corners_on then set_corners grid
in
let run corners_on =
  let lights = ref (Array.(map copy) init) in
  for _ = 1 to 100 do
    step !lights corners_on
  done;
  Array.copy !lights
in

let total_on grid =
  Array.fold_left ( + ) 0
    (Array.map (fun row -> Array.fold_left ( + ) 0 row) grid)
in

Printf.printf "First: %d\n" (total_on (run false));
Printf.printf "Second: %d\n" (total_on (run true))
