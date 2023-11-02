package main

import (
	"bufio"
	"fmt"
	"os"
)

func memsize(line string) (res int) {
	for i := 1; i < len(line)-1; i++ {
		if line[i] == '\\' && line[i+1] == 'x' {
			i += 3
		} else if line[i] == '\\' {
			i++
		}
		res++
	}
	return
}

func encoded(line string) (res int) {
	for i := 0; i < len(line); i++ {
		if line[i] == '"' || line[i] == '\\' {
			res++
		}
		res++
	}
	res += 2
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	first := 0
	second := 0
	for scanner.Scan() {
		line := scanner.Text()
		first += len(line) - memsize(line)
		second += encoded(line) - len(line)
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
