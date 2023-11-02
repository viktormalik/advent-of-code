package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, _ := os.ReadFile("input")
	split := strings.Fields(string(input))
	row, _ := strconv.Atoi(strings.TrimRight(split[15], ","))
	col, _ := strconv.Atoi(strings.TrimRight(split[17], "."))

	code := 20151125
	r := 1
	c := 1
	for {
		if r == 1 {
			r = c + 1
			c = 1
		} else {
			r--
			c++
		}
		code = (code * 252533) % 33554393
		if r == row && c == col {
			fmt.Println("First:", code)
			return
		}
	}
}
