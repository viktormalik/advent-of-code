package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func deliver(houses []int, elf, presents, limit int) {
	var upper int
	if limit != 0 {
		upper = min(elf*limit+1, len(houses))
	} else {
		upper = len(houses)
	}
	for e := elf; e < upper; e += elf {
		houses[e] += elf * presents
	}
}

func run(target, presents, limit int) int {
	houses := make([]int, target/2)
	for elf := 1; elf < target/2; elf++ {
		deliver(houses, elf, presents, limit)
	}
	return slices.IndexFunc(houses, func(h int) bool { return h >= target })
}

func main() {
	input, _ := os.ReadFile("input")
	target, _ := strconv.Atoi(strings.TrimSpace(string(input)))

	fmt.Println("First:", run(target, 10, 0))
	fmt.Println("Second:", run(target, 11, 50))
}
