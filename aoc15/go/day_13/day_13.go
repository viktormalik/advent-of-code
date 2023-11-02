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

func find(people []string, happiness map[string]int) int {
	perms := combin.Permutations(len(people), len(people))
	sittings := []int{}
	for _, perm := range perms {
		val := 0
		for i := 0; i < len(perm)-1; i++ {
			val += happiness[people[perm[i]]+people[perm[i+1]]]
			val += happiness[people[perm[i+1]]+people[perm[i]]]
		}
		val += happiness[people[perm[0]]+people[perm[len(perm)-1]]]
		val += happiness[people[perm[len(perm)-1]]+people[perm[0]]]
		sittings = append(sittings, val)
	}

	return slices.Max(sittings)
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	people := []string{}
	happiness := make(map[string]int)

	for scanner.Scan() {
		split := strings.Fields(scanner.Text())
		p1 := split[0]
		p2 := strings.TrimRight(split[10], ".")
		people = collections.SetAppend(people, p1)
		people = collections.SetAppend(people, p2)
		h, _ := strconv.Atoi(split[3])
		if split[2] == "lose" {
			h = -h
		}
		happiness[p1+p2] = h
	}

	fmt.Println("First:", find(people, happiness))

	for _, p := range people {
		happiness[p+"Me"] = 0
		happiness["Me"+p] = 0
	}
	people = append(people, "Me")
	fmt.Println("Second:", find(people, happiness))
}
