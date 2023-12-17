package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strings"
)

const (
	LEFT  = 0
	UP    = 1
	RIGHT = 2
	DOWN  = 3
)

func left(dir int) int {
	return ((dir-1)%4 + 4) % 4
}

func right(dir int) int {
	return (dir + 1) % 4
}

type State struct {
	Row, Col, Dir, Straight int
}

func validFunc(grid [][]int) func(State) bool {
	return func(state State) bool {
		return state.Row >= 0 && state.Row < len(grid) &&
			state.Col >= 0 && state.Col < len(grid[state.Row])
	}
}

func move(state State, dir int) (result State) {
	switch dir {
	case LEFT:
		result = State{state.Row, state.Col - 1, dir, state.Straight}
	case UP:
		result = State{state.Row - 1, state.Col, dir, state.Straight}
	case RIGHT:
		result = State{state.Row, state.Col + 1, dir, state.Straight}
	case DOWN:
		result = State{state.Row + 1, state.Col, dir, state.Straight}
	}
	if dir == state.Dir {
		result.Straight++
	} else {
		result.Straight = 1
	}
	return
}

func next(state State, grid [][]int, minStraight, maxStraight int) (result []State) {
	states := []State{}
	if state.Straight < maxStraight {
		states = append(states, move(state, state.Dir))
	}
	if state.Straight >= minStraight {
		states = append(states, move(state, left(state.Dir)))
		states = append(states, move(state, right(state.Dir)))
	}
	return collections.Filter(states, validFunc(grid))
}

func final(state State, grid [][]int, minStraight int) bool {
	return state.Row == len(grid)-1 && state.Col == len(grid[0])-1 &&
		state.Straight >= minStraight
}

func findMin(lastMin int, todo map[int][]State) int {
	result := lastMin
	for {
		if states, ok := todo[result]; ok && len(states) != 0 {
			return result
		}
		result++
	}
	return -1
}

func shortest(grid [][]int, minStraight, maxStraight int) int {
	s1 := State{0, 0, RIGHT, 0}
	s2 := State{0, 0, DOWN, 0}
	states := map[State]int{s1: 0, s2: 0}
	todo := map[int][]State{0: {s1, s2}}
	minLoss := 0
	for len(todo) > 0 {
		minLoss = findMin(minLoss, todo)
		current := todo[minLoss][0]
		todo[minLoss] = todo[minLoss][1:]

		if final(current, grid, minStraight) {
			return states[current]
		}

		for _, n := range next(current, grid, minStraight, maxStraight) {
			loss := states[current] + grid[n.Row][n.Col]
			if l, ok := states[n]; !ok || loss < l {
				states[n] = loss
				todo[loss] = append(todo[loss], n)
			}
		}
	}
	return 0
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	grid := [][]int{}
	for scanner.Scan() {
		row := collections.ParseInts(strings.Split(scanner.Text(), ""))
		grid = append(grid, row)
	}
	fmt.Println("First:", shortest(grid, 1, 3))
	fmt.Println("Second:", shortest(grid, 4, 10))
}
