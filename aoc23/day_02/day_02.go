package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	limits := map[string]int{"red": 12, "green": 13, "blue": 14}
	first := 0
	second := 0
	for scanner.Scan() {
		game := strings.Split(scanner.Text(), ": ")
		n, _ := strconv.Atoi(game[0][5:])
		draws := strings.Split(game[1], "; ")

		mins := map[string]int{"red": 0, "green": 0, "blue": 0}
		possible := true
		for _, d := range draws {
			stones := strings.Split(d, ", ")
			for _, s := range stones {
				color := strings.Fields(s)
				n, _ := strconv.Atoi(color[0])
				if n > mins[color[1]] {
					mins[color[1]] = n
				}
				if n > limits[color[1]] {
					possible = false
				}
			}
		}

		if possible {
			first += n
		}
		second += mins["red"] * mins["green"] * mins["blue"]
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
