package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

func value(w string, wires map[string]chan uint16) uint16 {
	v, err := strconv.Atoi(w)
	if err == nil {
		return uint16(v)
	} else {
		v := <-wires[w]
		return v
	}
}

func eval(name, rule string, wires map[string]chan uint16) {
	var val uint16
	split := strings.Fields(rule)
	if len(split) == 1 {
		val = value(split[0], wires)
	} else if split[0] == "NOT" {
		val = ^value(split[1], wires)
	} else {
		x := value(split[0], wires)
		y := value(split[2], wires)
		switch split[1] {
		case "AND":
			val = x & y
		case "OR":
			val = x | y
		case "LSHIFT":
			val = x << y
		case "RSHIFT":
			val = x >> y
		}
	}
	for {
		wires[name] <- val
	}
}

func run(rules map[string]string, wires map[string]chan uint16) uint16 {
	for name, rule := range rules {
		go eval(name, rule, wires)
	}
	res := <-wires["a"]
	return res
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	rules := make(map[string]string)
	wires := make(map[string]chan uint16)
	for scanner.Scan() {
		rule := strings.Split(scanner.Text(), " -> ")
		rules[rule[1]] = rule[0]
		wires[rule[1]] = make(chan uint16)
	}

	first := run(rules, wires)
	fmt.Println("First:", first)

	time.Sleep(100 * time.Millisecond)

	rules["b"] = strconv.Itoa(int(first))
	for w := range wires {
		wires[w] = make(chan uint16)
	}
	second := run(rules, wires)
	fmt.Println("Second:", second)
}
