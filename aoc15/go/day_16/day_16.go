package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Sue map[string]int

func parse(line string) Sue {
	str := line[strings.IndexByte(line, ':')+2:]
	things := strings.Split(str, ", ")
	sue := make(map[string]int)
	for _, thing := range things {
		ts := strings.Split(thing, ": ")
		n, _ := strconv.Atoi(ts[1])
		sue[ts[0]] = n
	}
	return sue
}

func firstCheck(sue, sender Sue) bool {
	for t, n := range sue {
		if sender[t] != n {
			return false
		}
	}
	return true
}

func secondCheck(sue, sender Sue) bool {
	for t, n := range sue {
		if t == "cats" || t == "trees" {
			if n <= sender[t] {
				return false
			}
		} else if t == "pomeranians" || t == "goldfish" {
			if n >= sender[t] {
				return false
			}
		} else if sender[t] != n {
			return false
		}
	}
	return true
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	sender := Sue{
		"children":    3,
		"cats":        7,
		"samoyeds":    2,
		"pomeranians": 3,
		"akitas":      0,
		"vizslas":     0,
		"goldfish":    5,
		"trees":       3,
		"cars":        2,
		"perfumes":    1,
	}

	i := 1
	for scanner.Scan() {
		sue := parse(scanner.Text())
		if firstCheck(sue, sender) {
			fmt.Println("First:", i)
		}
		if secondCheck(sue, sender) {
			fmt.Println("Second:", i)
		}
		i++
	}
}
