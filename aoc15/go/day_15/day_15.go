package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"gonum.org/v1/gonum/stat/combin"
)

type Ingredient []int

func parse(line string) Ingredient {
	split := strings.Fields(line)
	cap, _ := strconv.Atoi(strings.TrimRight(split[2], ","))
	dur, _ := strconv.Atoi(strings.TrimRight(split[4], ","))
	fla, _ := strconv.Atoi(strings.TrimRight(split[6], ","))
	tex, _ := strconv.Atoi(strings.TrimRight(split[8], ","))
	cal, _ := strconv.Atoi(split[10])
	return Ingredient{cap, dur, fla, tex, cal}
}

func evalProp(recipe []int, ingredients []Ingredient, prop int) int {
	score := 0
	for i, spoons := range recipe {
		score += spoons * ingredients[i][prop]
	}
	rem := 100 - collections.Sum(recipe)
	score += rem * ingredients[len(ingredients)-1][prop]
	if score < 0 {
		return 0
	}
	return score
}

func eval(recipe []int, ingredients []Ingredient) int {
	score := 1
	for prop := 0; prop < 4; prop++ {
		score *= evalProp(recipe, ingredients, prop)
	}
	return score
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	ingredients := []Ingredient{}
	for scanner.Scan() {
		ingredients = append(ingredients, parse(scanner.Text()))
	}

	lens := make([]int, len(ingredients)-1)
	for i := 0; i < len(lens); i++ {
		lens[i] = 100
	}
	recipes := combin.Cartesian(lens)

	first := 0
	second := 0
	for _, r := range recipes {
		score := eval(r, ingredients)
		if score > first {
			first = score
		}
		if evalProp(r, ingredients, 4) == 500 && score > second {
			second = score
		}
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
