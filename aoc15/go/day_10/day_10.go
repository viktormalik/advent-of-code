package main

import (
	"fmt"
	"os"
)

func apply(nums []byte) []byte {
	res := make([]byte, 0, 2*len(nums))
	for i := 0; i < len(nums); {
		n := 0
		for i+n < len(nums) && nums[i+n] == nums[i] {
			n++
		}
		res = append(res, byte(n), nums[i])
		i += n
	}
	return res
}

func run(initNums []byte, iters int) int {
	nums := initNums
	for i := 0; i < iters; i++ {
		nums = apply(nums)
	}
	return len(nums)
}

func main() {
	input, _ := os.ReadFile("input")

	nums := make([]byte, len(input)-1)
	for i, c := range input {
		if c >= '0' && c <= '9' {
			nums[i] = c - '0'
		}
	}

	fmt.Println("First:", run(nums, 40))
	fmt.Println("Second:", run(nums, 50))
}
