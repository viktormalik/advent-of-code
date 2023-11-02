package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
)

func on(row, col int, grid *[100][100]int) int {
	if row >= 0 && row < 100 && col >= 0 && col < 100 {
		return grid[row][col]
	}
	return 0
}

func neighs(row, col int, grid *[100][100]int) int {
	return on(row-1, col-1, grid) +
		on(row-1, col, grid) +
		on(row-1, col+1, grid) +
		on(row, col-1, grid) +
		on(row, col+1, grid) +
		on(row+1, col-1, grid) +
		on(row+1, col, grid) +
		on(row+1, col+1, grid)
}

func run(initGrid [100][100]int, cornersOn bool) int {
	grid := initGrid
	for s := 0; s < 100; s++ {
		newGrid := grid
		for r := 0; r < 100; r++ {
			for c := 0; c < 100; c++ {
				n := neighs(r, c, &grid)
				if grid[r][c] == 1 && n != 2 && n != 3 {
					newGrid[r][c] = 0
				} else if grid[r][c] == 0 && n == 3 {
					newGrid[r][c] = 1
				}
			}
		}
		grid = newGrid
		if cornersOn {
			grid[0][0] = 1
			grid[0][99] = 1
			grid[99][0] = 1
			grid[99][99] = 1
		}
	}

	sum := 0
	for _, row := range grid {
		sum += collections.Sum(row[:])
	}
	return sum
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	grid := [100][100]int{}
	r := 0
	for scanner.Scan() {
		line := scanner.Text()
		for c, x := range line {
			if x == '#' {
				grid[r][c] = 1
			}
		}
		r++
	}

	fmt.Println("First:", run(grid, false))
	fmt.Println("Second:", run(grid, true))
}
