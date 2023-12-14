package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func rollNorth(rocks [][]byte) {
	for row := 0; row < len(rocks); row++ {
		for col := 0; col < len(rocks[row]); col++ {
			if rocks[row][col] == 'O' {
				r := row
				for r > 0 && rocks[r-1][col] == '.' {
					r--
				}
				if r != row {
					rocks[r][col] = 'O'
					rocks[row][col] = '.'
				}
			}
		}
	}
}

func rollSouth(rocks [][]byte) {
	for row := len(rocks) - 1; row >= 0; row-- {
		for col := 0; col < len(rocks[row]); col++ {
			if rocks[row][col] == 'O' {
				r := row
				for r < len(rocks)-1 && rocks[r+1][col] == '.' {
					r++
				}
				if r != row {
					rocks[r][col] = 'O'
					rocks[row][col] = '.'
				}
			}
		}
	}
}

func rollWest(rocks [][]byte) {
	for col := 0; col < len(rocks[0]); col++ {
		for row := 0; row < len(rocks); row++ {
			if rocks[row][col] == 'O' {
				c := col
				for c > 0 && rocks[row][c-1] == '.' {
					c--
				}
				if c != col {
					rocks[row][c] = 'O'
					rocks[row][col] = '.'
				}
			}
		}
	}
}

func rollEast(rocks [][]byte) {
	for col := len(rocks[0]) - 1; col >= 0; col-- {
		for row := 0; row < len(rocks); row++ {
			if rocks[row][col] == 'O' {
				c := col
				for c < len(rocks)-1 && rocks[row][c+1] == '.' {
					c++
				}
				if c != col {
					rocks[row][c] = 'O'
					rocks[row][col] = '.'
				}
			}
		}
	}
}

func weight(rocks [][]byte) (result int) {
	for r, row := range rocks {
		for _, rock := range row {
			if rock == 'O' {
				result += len(rocks) - r
			}
		}
	}
	return
}

func serialize(rocks [][]byte) string {
	var sb strings.Builder
	for _, row := range rocks {
		sb.Write(row)
	}
	return sb.String()
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	rocks := [][]byte{}
	for scanner.Scan() {
		rocks = append(rocks, []byte(scanner.Text()))
	}

	first := rocks
	rollNorth(first)
	fmt.Println("First:", weight(first))

	seen := map[string]int{}
	n := 1000000000
	for i := 0; i < n; i++ {
		rollNorth(rocks)
		rollWest(rocks)
		rollSouth(rocks)
		rollEast(rocks)
		key := serialize(rocks)
		if last, ok := seen[key]; ok {
			period := i - last
			mod := (n - i) % period
			i = n - mod
		} else {
			seen[key] = i
		}
	}
	fmt.Println("Second:", weight(rocks))
}
