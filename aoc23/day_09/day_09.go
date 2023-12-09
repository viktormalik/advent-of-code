package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strings"
)

func next(nums []int, diff int) int {
	return nums[len(nums)-1] + diff
}

func prev(nums []int, diff int) int {
	return nums[0] - diff
}

func history(nums []int, dir func([]int, int) int) int {
	if collections.All(nums, func(n int) bool {
		return n == 0
	}) {
		return 0
	} else {
		diffs := []int{}
		for i := 0; i < len(nums)-1; i++ {
			diffs = append(diffs, nums[i+1]-nums[i])
		}
		return dir(nums, history(diffs, dir))
	}
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	first := 0
	second := 0
	for scanner.Scan() {
		nums := collections.ParseInts(strings.Fields(scanner.Text()))
		first += history(nums, next)
		second += history(nums, prev)
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
