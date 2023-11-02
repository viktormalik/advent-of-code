package main

import (
	"fmt"
	"os"
)

type House struct {
	x, y int
}

func deliver(input []byte, Houses map[House]bool, start, step int) {
	pos := House{0, 0}
	Houses[pos] = true
	for i := start; i < len(input); i += step {
		switch input[i] {
		case '^':
			pos = House{pos.x, pos.y + 1}
		case 'v':
			pos = House{pos.x, pos.y - 1}
		case '>':
			pos = House{pos.x + 1, pos.y}
		case '<':
			pos = House{pos.x - 1, pos.y}
		}
		Houses[pos] = true
	}
}

func main() {
	input, _ := os.ReadFile("input")

	first := make(map[House]bool)
	deliver(input, first, 0, 1)
	second := make(map[House]bool)
	deliver(input, second, 0, 2)
	deliver(input, second, 1, 2)
	fmt.Println("First:", len(first))
	fmt.Println("Second:", len(second))
}
