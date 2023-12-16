package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
)

const (
	LEFT  = 0
	RIGHT = 1
	UP    = 2
	DOWN  = 3
)

type Start struct {
	row, col, dir int
}

func propagate(grid [][]byte, row, col, dir int, energized [][]int, visited map[Start]bool) {
	if visited[Start{row, col, dir}] {
		return
	}
	visited[Start{row, col, dir}] = true
	switch dir {
	case LEFT:
		for c := col - 1; c >= 0; c-- {
			energized[row][c] = 1
			switch grid[row][c] {
			case '/':
				propagate(grid, row, c, DOWN, energized, visited)
				return
			case '\\':
				propagate(grid, row, c, UP, energized, visited)
				return
			case '|':
				propagate(grid, row, c, UP, energized, visited)
				propagate(grid, row, c, DOWN, energized, visited)
				return
			}
		}
	case RIGHT:
		for c := col + 1; c < len(grid[row]); c++ {
			energized[row][c] = 1
			switch grid[row][c] {
			case '/':
				propagate(grid, row, c, UP, energized, visited)
				return
			case '\\':
				propagate(grid, row, c, DOWN, energized, visited)
				return
			case '|':
				propagate(grid, row, c, UP, energized, visited)
				propagate(grid, row, c, DOWN, energized, visited)
				return
			}
		}
	case UP:
		for r := row - 1; r >= 0; r-- {
			energized[r][col] = 1
			switch grid[r][col] {
			case '/':
				propagate(grid, r, col, RIGHT, energized, visited)
				return
			case '\\':
				propagate(grid, r, col, LEFT, energized, visited)
				return
			case '-':
				propagate(grid, r, col, LEFT, energized, visited)
				propagate(grid, r, col, RIGHT, energized, visited)
				return
			}
		}
	case DOWN:
		for r := row + 1; r < len(grid); r++ {
			energized[r][col] = 1
			switch grid[r][col] {
			case '/':
				propagate(grid, r, col, LEFT, energized, visited)
				return
			case '\\':
				propagate(grid, r, col, RIGHT, energized, visited)
				return
			case '-':
				propagate(grid, r, col, LEFT, energized, visited)
				propagate(grid, r, col, RIGHT, energized, visited)
				return
			}
		}
	}
}

func run(grid [][]byte, row, col, dir int) int {
	energized := make([][]int, len(grid))
	for row := range energized {
		energized[row] = make([]int, len(grid[row]))
	}
	visited := map[Start]bool{}
	propagate(grid, row, col, dir, energized, visited)
	return collections.SumFunc(energized, func(row []int) int {
		return collections.Sum(row)
	})
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	grid := [][]byte{}
	for scanner.Scan() {
		grid = append(grid, []byte(scanner.Text()))
	}
	fmt.Println("First:", run(grid, 0, -1, RIGHT))

	second := 0
	for row := 0; row < len(grid); row++ {
		second = max(second, run(grid, row, -1, RIGHT))
		second = max(second, run(grid, row, len(grid[row]), LEFT))
	}
	for col := 0; col < len(grid[0]); col++ {
		second = max(second, run(grid, -1, col, DOWN))
		second = max(second, run(grid, len(grid), col, UP))
	}
	fmt.Println("Second:", second)
}
