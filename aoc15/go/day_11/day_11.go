package main

import (
	"fmt"
	"os"
	"slices"
)

func next(pass []byte) {
	for i := len(pass) - 1; i >= 0; i-- {
		if pass[i] == 'z' {
			pass[i] = 'a'
		} else {
			pass[i]++
			break
		}
	}
}

func increasing(pass []byte) bool {
	for i := 2; i < len(pass); i++ {
		if pass[i] == pass[i-1]+1 && pass[i] == pass[i-2]+2 {
			return true
		}
	}
	return false
}

func invalid(pass []byte) bool {
	return slices.ContainsFunc(pass, func(c byte) bool {
		return c == 'i' || c == 'l' || c == 'o'
	})
}

func twopairs(pass []byte) bool {
	count := 0
	for i := 1; i < len(pass); i++ {
		if pass[i] == pass[i-1] {
			i++
			count++
		}
	}
	return count >= 2
}

func valid(pass []byte) bool {
	return increasing(pass) && !invalid(pass) && twopairs(pass)
}

func find(pass []byte) []byte {
	for {
		next(pass)
		if valid(pass) {
			return pass
		}
	}
}

func main() {
	input, _ := os.ReadFile("input")
	pass := input[:len(input)-1]

	fmt.Println("First:", string(find(pass)))
	fmt.Println("Second:", string(find(pass)))
}
