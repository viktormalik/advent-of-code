package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

const (
	LEFT  = 0
	UP    = 1
	RIGHT = 2
	DOWN  = 3
)

type Point struct {
	X, Y int
}

func opposite(dir int) int {
	switch dir {
	case LEFT:
		return RIGHT
	case UP:
		return DOWN
	case RIGHT:
		return LEFT
	case DOWN:
		return UP
	}
	return dir
}

func get(pt Point, grid [][]byte) byte {
	return grid[pt.X][pt.Y]
}

func move(pt Point, dir int) Point {
	switch dir {
	case LEFT:
		return Point{pt.X, pt.Y - 1}
	case UP:
		return Point{pt.X - 1, pt.Y}
	case RIGHT:
		return Point{pt.X, pt.Y + 1}
	case DOWN:
		return Point{pt.X + 1, pt.Y}
	default:
		return pt
	}
}

func first(start Point, grid [][]byte) (Point, int) {
	var left, up, right, down bool
	if l := get(move(start, LEFT), grid); l == '-' || l == 'L' || l == 'F' {
		left = true
	}
	if r := get(move(start, RIGHT), grid); r == '-' || r == '7' || r == 'J' {
		right = true
	}
	if u := get(move(start, UP), grid); u == '|' || u == '7' || u == 'F' {
		up = true
	}
	if d := get(move(start, DOWN), grid); d == '|' || d == 'L' || d == 'J' {
		down = true
	}

	var pipe byte
	var dir int
	switch {
	case left && right:
		pipe = '-'
		dir = RIGHT
	case up && down:
		pipe = '|'
		dir = DOWN
	case up && right:
		pipe = 'L'
		dir = RIGHT
	case up && left:
		pipe = 'J'
		dir = LEFT
	case down && right:
		pipe = 'F'
		dir = RIGHT
	case down && left:
		pipe = '7'
		dir = LEFT
	}
	grid[start.X][start.Y] = pipe
	return move(start, dir), opposite(dir)
}

func next(pt Point, from int, grid [][]byte) (Point, int) {
	var dir int
	switch get(pt, grid) {
	case '|', '-':
		dir = opposite(from)
	case 'L':
		if from == RIGHT {
			dir = UP
		} else {
			dir = RIGHT
		}
	case 'J':
		if from == LEFT {
			dir = UP
		} else {
			dir = LEFT
		}
	case '7':
		if from == LEFT {
			dir = DOWN
		} else {
			dir = LEFT
		}
	case 'F':
		if from == DOWN {
			dir = RIGHT
		} else {
			dir = DOWN
		}
	}
	return move(pt, dir), opposite(dir)
}

func crosses(from Point, grid [][]byte, loop map[Point]bool) (result int) {
	pt := from
	lastTurn := byte('.')
	for pt.Y >= 0 {
		if loop[pt] {
			pipe := get(pt, grid)
			if pipe == '|' || pipe == 'L' && lastTurn == '7' || pipe == 'F' && lastTurn == 'J' {
				result++
			}
			if pipe == 'L' || pipe == 'F' {
				lastTurn = '.'
			} else if pipe == 'J' || pipe == '7' {
				lastTurn = pipe
			}
		}
		pt = move(pt, LEFT)
	}
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	grid := [][]byte{}
	for scanner.Scan() {
		grid = append(grid, []byte(scanner.Text()))
	}
	var start Point
	for r, row := range grid {
		c := slices.Index(row, 'S')
		if c >= 0 {
			start = Point{r, c}
			break
		}
	}

	pos, from := first(start, grid)
	loop := map[Point]bool{pos: true}
	for pos != start {
		pos, from = next(pos, from, grid)
		loop[pos] = true
	}
	fmt.Println("First:", len(loop)/2)

	second := 0
	for x := 0; x < len(grid); x++ {
		for y := 0; y < len(grid[x]); y++ {
			pt := Point{x, y}
			if !loop[pt] && crosses(pt, grid, loop)%2 == 1 {
				second++
			}
		}
	}
	fmt.Println("Second:", second)
}
