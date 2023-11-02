package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	"gonum.org/v1/gonum/stat/combin"
)

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	containers := []int{}
	for scanner.Scan() {
		c, _ := strconv.Atoi(scanner.Text())
		containers = append(containers, c)
	}

	first := 0
	second := 0
	for n := 0; n < len(containers); n++ {
		if second == 0 && first != 0 {
			second = first
		}
		perms := combin.Combinations(len(containers), n)
		for _, perm := range perms {
			sum := 0
			for _, p := range perm {
				sum += containers[p]
			}
			if sum == 150 {
				first++
			}
		}
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
