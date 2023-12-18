package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Inst struct {
	Dir string
	N   int
}

var dirs = map[string]string{"0": "R", "1": "D", "2": "L", "3": "U"}

func mod(a, b int) int {
	return (a%b + b) % b
}

func nextDir(i int, plan []Inst) string {
	return plan[mod(i+1, len(plan))].Dir
}

func prevDir(i int, plan []Inst) string {
	return plan[mod(i-1, len(plan))].Dir
}

func area(plan []Inst) (result int) {
	row := 0
	col := 0
	for i := 0; i < len(plan); i++ {
		oldRow := row
		oldCol := col
		switch plan[i].Dir {
		case "R":
			col += plan[i].N
			if nextDir(i, plan) == "D" {
				col++
			}
			if prevDir(i, plan) == "D" {
				col--
			}
		case "L":
			col -= plan[i].N
			if nextDir(i, plan) == "U" {
				col--
			}
			if prevDir(i, plan) == "U" {
				col++
			}
		case "D":
			row += plan[i].N
			if nextDir(i, plan) == "L" {
				row++
			}
			if prevDir(i, plan) == "L" {
				row--
			}
		case "U":
			row -= plan[i].N
			if nextDir(i, plan) == "R" {
				row--
			}
			if prevDir(i, plan) == "R" {
				row++
			}
		}
		result += oldRow*col - row*oldCol
	}
	return int(math.Abs(float64(result))) / 2
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	firstPlan := []Inst{}
	secondPlan := []Inst{}
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())

		firstN, _ := strconv.Atoi(fields[1])
		firstPlan = append(firstPlan, Inst{fields[0], firstN})

		secondN, _ := strconv.ParseInt(fields[2][2:7], 16, 32)
		secondPlan = append(secondPlan, Inst{dirs[fields[2][7:8]], int(secondN)})
	}
	fmt.Println("First:", area(firstPlan))
	fmt.Println("Second:", area(secondPlan))
}
