package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strings"
)

func threevowels(str string) bool {
	return collections.CountFunc([]rune(str), vowel) >= 3
}

func twice(str string) bool {
	for i := 0; i < len(str); i++ {
		if i > 0 && str[i] == str[i-1] {
			return true
		}
	}
	return false
}

func vowel(c rune) bool {
	return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

func badpairs(str string) bool {
	return strings.Contains(str, "ab") || strings.Contains(str, "cd") ||
		strings.Contains(str, "pq") || strings.Contains(str, "xy")
}

func niceFirst(str string) bool {
	return threevowels(str) && twice(str) && !badpairs(str)
}

func twicepair(str string) bool {
	for i := 0; i < len(str); i++ {
		if i >= 2 && strings.Contains(str[i:], string(str[i-2])+string(str[i-1])) {
			return true
		}
	}
	return false
}

func triplet(str string) bool {
	for i := 0; i < len(str); i++ {
		if i >= 2 && str[i] == str[i-2] {
			return true
		}
	}
	return false
}

func niceSecond(str string) bool {
	return twicepair(str) && triplet(str)
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)
	strings := []string{}
	for scanner.Scan() {
		strings = append(strings, scanner.Text())
	}

	fmt.Println("First:", collections.CountFunc(strings, niceFirst))
	fmt.Println("Second:", collections.CountFunc(strings, niceSecond))
}
