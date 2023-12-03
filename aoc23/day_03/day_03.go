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

func isDigit(r rune) bool {
	return r >= '0' && r <= '9'
}

func isValid[T any](grid [][]T, p Pos) bool {
	return p.Row >= 0 && p.Row < len(grid) && p.Col >= 0 && p.Col < len(grid[p.Row])
}

func neighbors[T any](grid [][]T, p Pos) (res []T) {
	neighs := []Pos{
		{p.Row - 1, p.Col - 1}, {p.Row - 1, p.Col}, {p.Row - 1, p.Col + 1},
		{p.Row, p.Col - 1}, {p.Row, p.Col + 1},
		{p.Row + 1, p.Col - 1}, {p.Row + 1, p.Col}, {p.Row + 1, p.Col + 1},
	}
	for _, n := range neighs {
		if isValid(grid, n) {
			res = append(res, grid[n.Row][n.Col])
		}
	}
	return
}

func hasNeighborSymbol(engine [][]rune, p Pos) bool {
	return collections.Any(neighbors(engine, p), func(r rune) bool {
		return !isDigit(r) && r != '.'
	})
}

func neighborNums(nums [][]int, p Pos) (res []int) {
	for _, n := range neighbors(nums, p) {
		if n > 0 {
			res = collections.SetAppend(res, n)
		}
	}
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	engine := [][]rune{}

	row := 0
	for scanner.Scan() {
		engine = append(engine, []rune{})
		for _, x := range scanner.Text() {
			engine[row] = append(engine[row], x)
		}
		row++
	}

	first := 0
	n := 0
	nPos := []Pos{}
	part := false
	numbers := make([][]int, len(engine))
	for r := 0; r < len(engine); r++ {
		numbers[r] = make([]int, len(engine[r]))
		for c := 0; c < len(engine[r]); c++ {
			if isDigit(engine[r][c]) {
				n = n*10 + int(engine[r][c]-'0')
				nPos = append(nPos, Pos{r, c})
				if hasNeighborSymbol(engine, Pos{r, c}) {
					part = true
				}
			} else if n != 0 {
				if part {
					first += n
				}
				for _, p := range nPos {
					numbers[p.Row][p.Col] = n
				}
				n = 0
				nPos = []Pos{}
				part = false
			}
		}
	}
	fmt.Println("First:", first)

	second := 0
	for r := 0; r < len(engine); r++ {
		for c := 0; c < len(engine[r]); c++ {
			if engine[r][c] == '*' {
				ns := neighborNums(numbers, Pos{r, c})
				if len(ns) == 2 {
					second += ns[0] * ns[1]
				}
			}
		}
	}
	fmt.Println("Second:", second)
}
