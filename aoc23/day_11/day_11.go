package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
)

type Galaxy struct {
	Row, Col int
}

func rowDst(g1, g2 Galaxy, universe [][]byte, expansion int) (result int) {
	for r := min(g1.Row, g2.Row) + 1; r <= max(g1.Row, g2.Row); r++ {
		if collections.All(universe[r], func(b byte) bool {
			return b == '.'
		}) {
			result += expansion
		} else {
			result += 1
		}
	}
	return
}

func colDst(g1, g2 Galaxy, universe [][]byte, expansion int) (result int) {
	for c := min(g1.Col, g2.Col) + 1; c <= max(g1.Col, g2.Col); c++ {
		if collections.All(universe, func(row []byte) bool {
			return row[c] == '.'
		}) {
			result += expansion
		} else {
			result += 1
		}
	}
	return
}

func dst(g1, g2 Galaxy, universe [][]byte, expansion int) int {
	return rowDst(g1, g2, universe, expansion) + colDst(g1, g2, universe, expansion)
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	universe := [][]byte{}
	for scanner.Scan() {
		universe = append(universe, []byte(scanner.Text()))
	}

	galaxies := []Galaxy{}
	for r, row := range universe {
		for c, x := range row {
			if x == '#' {
				galaxies = append(galaxies, Galaxy{r, c})
			}
		}
	}

	first := 0
	second := 0
	for _, g1 := range galaxies {
		for _, g2 := range galaxies {
			first += dst(g1, g2, universe, 2)
			second += dst(g1, g2, universe, 1000000)
		}
	}
	fmt.Println("First:", first/2)
	fmt.Println("Second:", second/2)
}
