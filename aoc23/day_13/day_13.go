package main

import (
	"bufio"
	"fmt"
	"os"
)

func verticalDiffs(pattern []string, col int) (result int) {
	for i := 0; col-i >= 0 && col+i+1 < len(pattern[0]); i++ {
		for r := 0; r < len(pattern); r++ {
			if pattern[r][col-i] != pattern[r][col+i+1] {
				result++
			}
		}
	}
	return
}

func verticalMirror(pattern []string, smudges int) int {
	for col := 0; col < len(pattern[0])-1; col++ {
		if verticalDiffs(pattern, col) == smudges {
			return col + 1
		}
	}
	return -1
}

func horizontalDiffs(pattern []string, row int) (result int) {
	for i := 0; row-i >= 0 && row+i+1 < len(pattern); i++ {
		for c := 0; c < len(pattern[0]); c++ {
			if pattern[row-i][c] != pattern[row+i+1][c] {
				result++
			}
		}
	}
	return
}

func horizontalMirror(pattern []string, smudges int) int {
	for row := 0; row < len(pattern)-1; row++ {
		if horizontalDiffs(pattern, row) == smudges {
			return row + 1
		}
	}
	return -1
}

func findReflections(patterns [][]string, smudges int) (result int) {
	for _, pattern := range patterns {
		if col := verticalMirror(pattern, smudges); col >= 0 {
			result += col
		} else if row := horizontalMirror(pattern, smudges); row >= 0 {
			result += 100 * row
		}
	}
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	patterns := [][]string{}
	pattern := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			patterns = append(patterns, pattern)
			pattern = []string{}
		} else {
			pattern = append(pattern, line)
		}
	}
	patterns = append(patterns, pattern)

	fmt.Println("First:", findReflections(patterns, 0))
	fmt.Println("Second:", findReflections(patterns, 1))
}
