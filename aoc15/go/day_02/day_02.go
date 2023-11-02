package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Box struct {
	L, W, H int
}

func create(line string) Box {
	dims := strings.Split(line, "x")
	l, _ := strconv.Atoi(dims[0])
	w, _ := strconv.Atoi(dims[1])
	h, _ := strconv.Atoi(dims[2])
	return Box{l, w, h}
}

func (b Box) areas() []int {
	return []int{2 * b.L * b.W, 2 * b.W * b.H, 2 * b.H * b.L}
}

func (b Box) perimeters() []int {
	return []int{2 * (b.L + b.W), 2 * (b.W + b.H), 2 * (b.H + b.L)}
}

func (b Box) volume() int {
	return b.L * b.W * b.H
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	first := 0
	second := 0
	for scanner.Scan() {
		box := create(scanner.Text())
		first += collections.Sum(box.areas()) + slices.Min(box.areas())/2
		second += slices.Min(box.perimeters()) + box.volume()
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
