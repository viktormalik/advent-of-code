import std/strutils

type Word = proc(letters: seq[(int, int)]): bool

let input = readFile("input").strip.splitLines

proc check(r: int, c: int, l: char): bool =
  r >= 0 and r < input.len and c >= 0 and c < input[r].len and input[r][c] == l

proc xmas(letters: seq[(int, int)]): bool =
  check(letters[0][0], letters[0][1], 'X') and
  check(letters[1][0], letters[1][1], 'M') and
  check(letters[2][0], letters[2][1], 'A') and
  check(letters[3][0], letters[3][1], 'S')

proc mas(letters: seq[(int, int)]): bool =
  check(letters[0][0], letters[0][1], 'M') and
  check(letters[1][0], letters[1][1], 'A') and
  check(letters[2][0], letters[2][1], 'S')

proc right(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r, c + 1), (r, c + 2), (r, c + 3)])

proc left(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r, c - 1), (r, c - 2), (r, c - 3)])

proc up(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r - 1, c), (r - 2, c), (r - 3, c)])

proc down(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r + 1, c), (r + 2, c), (r + 3, c)])

proc right_down(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r + 1, c + 1), (r + 2, c + 2), (r + 3, c + 3)])

proc right_up(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r - 1, c + 1), (r - 2, c + 2), (r - 3, c + 3)])

proc left_down(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r + 1, c - 1), (r + 2, c - 2), (r + 3, c - 3)])

proc left_up(r: int, c: int, word: Word): bool =
  word(@[(r, c), (r - 1, c - 1), (r - 2, c - 2), (r - 3, c - 3)])

var first = 0
for r in 0..input.len - 1:
  for c in 0..input[r].len - 1:
    first += int(right(r, c, xmas)) + int(left(r, c, xmas)) +
             int(up(r, c, xmas)) + int(down(r, c, xmas)) +
             int(right_down(r, c, xmas)) + int(right_up(r, c, xmas)) +
             int(left_down(r, c, xmas)) + int(left_up(r, c, xmas))
echo "First: " & $first

var second = 0
for r in 0..input.len - 1:
  for c in 0..input[r].len - 1:
    if (right_down(r - 1, c - 1, mas) or left_up(r + 1, c + 1, mas)) and
       (left_down(r - 1, c + 1, mas) or right_up(r + 1, c - 1, mas)):
      inc(second)
echo "Second: " & $second
