package main

import (
	"fmt"
	"os"
)

func main() {
	input, _ := os.ReadFile("input")

	floor := 0
	second := 0
	for i, c := range input {
		switch c {
		case '(':
			floor++
		case ')':
			floor--
		}
		if floor < 0 && second == 0 {
			second = i + 1
		}
	}
	fmt.Println("First:", floor)
	fmt.Println("Second:", second)
}
