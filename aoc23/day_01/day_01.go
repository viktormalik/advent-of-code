package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func getWord(line string, i int) int {
	words := [10]string{
		"zero", "one", "two", "three", "four",
		"five", "six", "seven", "eight", "nine",
	}
	for n, w := range words {
		if strings.HasPrefix(line[i:], w) {
			return n
		}
	}
	return -1
}

func firstWord(line string) (int, int) {
	for i := 0; i < len(line); i++ {
		n := getWord(line, i)
		if n >= 0 {
			return i, n
		}
	}
	return -1, -1
}

func lastWord(line string) (int, int) {
	for i := len(line) - 1; i >= 0; i-- {
		n := getWord(line, i)
		if n >= 0 {
			return i, n
		}
	}
	return -1, -1
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	first := 0
	second := 0
	for scanner.Scan() {
		line := scanner.Text()

		id1 := strings.IndexAny(line, "0123456789")
		d1 := int(line[id1] - '0')
		id2 := strings.LastIndexAny(line, "0123456789")
		d2 := int(line[id2] - '0')
		iw1, w1 := firstWord(line)
		iw2, w2 := lastWord(line)

		first += 10*d1 + d2

		if iw1 < 0 || id1 < iw1 {
			second += 10 * d1
		} else {
			second += 10 * w1
		}

		if iw2 < 0 || id2 > iw2 {
			second += d2
		} else {
			second += w2
		}
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
