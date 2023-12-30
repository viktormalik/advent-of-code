package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
)

type Pos struct {
	Row, Col int
}

func neighs(pos Pos, grid [][]byte) []Pos {
	res := []Pos{
		{pos.Row + 1, pos.Col},
		{pos.Row - 1, pos.Col},
		{pos.Row, pos.Col + 1},
		{pos.Row, pos.Col - 1},
	}
	return collections.Filter(res, func(p Pos) bool {
		return p.Row >= 0 && p.Row < len(grid) &&
			p.Col >= 0 && p.Col < len(grid[p.Row]) &&
			grid[p.Row][p.Col] != '#'
	})
}

func allDsts(start Pos, grid [][]byte) map[Pos]int {
	queue := []Pos{start}
	result := map[Pos]int{start: 0}
	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]
		for _, n := range neighs(current, grid) {
			if _, ok := result[n]; !ok {
				result[n] = result[current] + 1
				queue = append(queue, n)
			}
		}
	}
	return result
}

func countFrom(from Pos, grid [][]byte, limit int) (odd, even int) {
	for pos, dst := range allDsts(from, grid) {
		if dst <= limit {
			if (pos.Row+pos.Col)%2 == 0 {
				even++
			} else {
				odd++
			}
		}
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
	mid := len(grid) / 2
	last := len(grid) - 1
	start := Pos{mid, mid}

	// 64 < 65 (mid) so we just count from start
	_, first := countFrom(start, grid, 64)
	fmt.Println("First:", first)

	second := 0
	steps := 26501365
	grids := (steps - len(grid)/2) / len(grid)

	// Full grids
	odd := (grids - 1) * (grids - 1)
	even := grids * grids
	oddFull, evenFull := countFrom(start, grid, len(grid))
	second += oddFull*odd + evenFull*even

	// Diamond corners (all are odd)
	top, _ := countFrom(Pos{last, mid}, grid, len(grid)-1)
	left, _ := countFrom(Pos{mid, 0}, grid, len(grid)-1)
	bottom, _ := countFrom(Pos{0, mid}, grid, len(grid)-1)
	right, _ := countFrom(Pos{mid, last}, grid, len(grid)-1)
	second += top + left + bottom + right

	// Edges (small are even, large are odd)
	_, topRightS := countFrom(Pos{last, 0}, grid, len(grid)/2-1)
	_, topLeftS := countFrom(Pos{last, last}, grid, len(grid)/2-1)
	_, bottomRightS := countFrom(Pos{0, 0}, grid, len(grid)/2-1)
	_, bottomLeftS := countFrom(Pos{0, last}, grid, len(grid)/2-1)
	second += (topRightS + topLeftS + bottomRightS + bottomLeftS) * grids

	topRightL, _ := countFrom(Pos{last, 0}, grid, len(grid)+len(grid)/2-1)
	topLeftL, _ := countFrom(Pos{last, last}, grid, len(grid)+len(grid)/2-1)
	bottomRightL, _ := countFrom(Pos{0, 0}, grid, len(grid)+len(grid)/2-1)
	bottomLeftL, _ := countFrom(Pos{0, last}, grid, len(grid)+len(grid)/2-1)
	second += (topRightL + topLeftL + bottomRightL + bottomLeftL) * (grids - 1)

	fmt.Println("Second:", second)
}
