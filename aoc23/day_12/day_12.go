package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strings"
)

func count(springs string, groups []int, cache map[string]int) (result int) {
	if len(springs) == 0 {
		return 0
	}
	if cnt, ok := cache[springs+string(len(groups))]; ok {
		return cnt
	}
	for shift := 0; shift <= len(springs)-collections.Sum(groups); shift++ {
		if shift > 0 && springs[shift-1] == '#' {
			break
		}
		if collections.Any([]byte(springs[shift:shift+groups[0]]), func(s byte) bool {
			return s == '.'
		}) {
			continue
		}
		if len(groups) == 1 {
			if collections.All([]byte(springs[shift+groups[0]:]), func(s byte) bool {
				return s != '#'
			}) {
				result++
			}
		} else {
			if springs[shift+groups[0]] != '#' {
				result += count(springs[shift+groups[0]+1:], groups[1:], cache)
			}
		}
	}
	cache[springs+string(len(groups))] = result
	return
}

func unfold(springs string, groups []int) (string, []int) {
	newSprings := springs
	newGroups := groups
	for i := 0; i < 4; i++ {
		newSprings += "?" + springs
		newGroups = append(newGroups, groups...)
	}
	return newSprings, newGroups
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	first := 0
	second := 0
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		groups := collections.ParseInts(strings.Split(fields[1], ","))
		first += count(fields[0], groups, map[string]int{})

		uSprings, uGroups := unfold(fields[0], groups)
		second += count(uSprings, uGroups, map[string]int{})
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
