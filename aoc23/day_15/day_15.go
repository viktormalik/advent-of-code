package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Lens struct {
	Label string
	Focal int
}

func hasLabel(label string) func(Lens) bool {
	return func(lens Lens) bool { return lens.Label == label }
}

func hash(str string) (result int) {
	for _, b := range []byte(str) {
		result += int(b)
		result *= 17
		result = result % 256
	}
	return
}

func power(boxes [][]Lens) (result int) {
	for b := range boxes {
		for l := range boxes[b] {
			result += (b + 1) * (l + 1) * boxes[b][l].Focal
		}
	}
	return
}

func main() {
	input, _ := os.ReadFile("input")
	boxes := make([][]Lens, 256)

	first := 0
	for _, step := range strings.Split(strings.TrimSpace(string(input)), ",") {
		first += hash(step)
		if strings.Contains(step, "-") {
			label := strings.TrimRight(step, "-")
			box := hash(label)
			boxes[box] = slices.DeleteFunc(boxes[box], hasLabel(label))
		} else {
			split := strings.Split(step, "=")
			label := split[0]
			box := hash(label)
			focal, _ := strconv.Atoi(split[1])
			lens := slices.IndexFunc(boxes[box], hasLabel(label))
			if lens >= 0 {
				boxes[box][lens].Focal = focal
			} else {
				boxes[box] = append(boxes[box], Lens{label, focal})
			}
		}
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", power(boxes))
}
