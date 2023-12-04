package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strings"
)

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	first := 0
	copies := map[int]int{}
	n := 0
	for scanner.Scan() {
		card := strings.Split(scanner.Text(), ": ")
		numbers := strings.Split(card[1], " | ")
		winning := strings.Fields(strings.TrimSpace(numbers[0]))
		have := strings.Fields(numbers[1])

		matching := collections.CountFunc(have, func(h string) bool {
			return slices.Contains(winning, h)
		})
		first += int(math.Pow(2, float64(matching-1)))

		copies[n]++
		for c := n + 1; c <= n+matching; c++ {
			copies[c] += copies[n]
		}
		n++
	}
	fmt.Println("First:", first)

	second := 0
	for c := 0; c < n; c++ {
		second += copies[c]
	}
	fmt.Println("Second:", second)
}
