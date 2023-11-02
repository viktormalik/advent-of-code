package main

import (
	"aoc/collections"
	"bufio"
	"cmp"
	"fmt"
	"os"
	"slices"
	"strconv"

	"gonum.org/v1/gonum/stat/combin"
)

func weight(group []int, pkgs []int) (res int) {
	for _, g := range group {
		res += pkgs[g]
	}
	return
}

func entanglement(group []int, pkgs []int) int {
	res := 1
	for _, g := range group {
		res *= pkgs[g]
	}
	return res
}

func split(pkgs []int, n int) int {
	target := collections.Sum(pkgs) / n

	candidates := [][]int{}
	for k := 1; true; k++ {
		groups := combin.Combinations(len(pkgs), k)
		candidates = collections.Filter(groups, func(g []int) bool {
			return weight(g, pkgs) == target
		})
		if len(candidates) > 0 {
			break
		}
	}

	result := slices.MinFunc(candidates, func(a, b []int) int {
		return cmp.Compare(entanglement(a, pkgs), entanglement(b, pkgs))
	})

	return entanglement(result, pkgs)
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	pkgs := []int{}
	for scanner.Scan() {
		p, _ := strconv.Atoi(scanner.Text())
		pkgs = append(pkgs, p)
	}
	fmt.Println("First:", split(pkgs, 3))
	fmt.Println("Second:", split(pkgs, 4))
}
