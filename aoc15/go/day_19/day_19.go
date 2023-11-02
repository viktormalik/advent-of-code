package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"regexp"
	"slices"
	"strings"
)

type Rule struct {
	React string
	Prods []string
}

func tryApply(mol []string, rule Rule, index int) []string {
	if mol[index] == rule.React {
		newMol := make([]string, 0, len(mol)+len(rule.Prods)-1)
		newMol = append(newMol, mol[:index]...)
		newMol = append(newMol, rule.Prods...)
		newMol = append(newMol, mol[index+1:]...)
		return newMol
	}
	return mol
}

func reduce(mol []string, rule Rule, index int) []string {
	newMol := make([]string, 0, len(mol)-len(rule.Prods)+1)
	newMol = append(newMol, mol[:index]...)
	newMol = append(newMol, rule.React)
	newMol = append(newMol, mol[index+len(rule.Prods):]...)
	return newMol
}

func reduceRL(mol []string, rules []Rule) []string {
	for i := len(mol) - 1; i >= 0; i-- {
		for _, rule := range rules {
			if i+len(rule.Prods) <= len(mol) &&
				slices.Equal(mol[i:i+len(rule.Prods)], rule.Prods) {
				return reduce(mol, rule, i)
			}
		}
	}
	return mol
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	re := regexp.MustCompile("[A-Z][a-z]*")

	var target []string
	rules := []Rule{}

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			scanner.Scan()
			line = scanner.Text()
			target = re.FindAllString(line, -1)
			break
		}
		rule := strings.Split(line, " => ")
		rules = append(rules, Rule{rule[0], re.FindAllString(rule[1], -1)})
	}

	first := []string{strings.Join(target, "")}
	for i := 0; i < len(target); i++ {
		for _, rule := range rules {
			first = collections.SetAppend(
				first,
				strings.Join(tryApply(target, rule, i), ""))
		}
	}
	fmt.Println("First:", len(first)-1)

	second := 0
	current := target
	for len(current) != 1 {
		current = reduceRL(current, rules)
		second++
	}
	fmt.Println("Second:", second)
}
