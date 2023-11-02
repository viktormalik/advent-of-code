package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"

	"gonum.org/v1/gonum/stat/combin"
)

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	locs := []string{}
	dst := make(map[string]int)
	for scanner.Scan() {
		split := strings.Fields(scanner.Text())
		x := split[0]
		y := split[2]
		d, _ := strconv.Atoi(split[4])
		locs = collections.SetAppend(locs, x)
		locs = collections.SetAppend(locs, y)
		dst[x+y] = d
		dst[y+x] = d
	}

	perms := combin.Permutations(len(locs), len(locs))
	routes := []int{}
	for _, perm := range perms {
		r := 0
		for i := 0; i < len(perm)-1; i++ {
			r += dst[locs[perm[i]]+locs[perm[i+1]]]
		}
		routes = append(routes, r)
	}
	first := slices.Min(routes)
	fmt.Println("First:", first)
	second := slices.Max(routes)
	fmt.Println("Second:", second)
}
