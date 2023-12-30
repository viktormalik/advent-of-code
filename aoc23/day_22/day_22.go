package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

type Point struct {
	X, Y, Z int
}

type Brick struct {
	Id     int
	Points []Point
}

type Stack [][][]bool

func (b Brick) fall() Brick {
	newPoints := collections.Map(b.Points, func(p Point) Point {
		return Point{p.X, p.Y, p.Z - 1}
	})
	return Brick{b.Id, newPoints}
}

func (stack Stack) remove(b Brick) {
	for _, p := range b.Points {
		stack[p.X][p.Y][p.Z] = false
	}
}

func (stack Stack) put(b Brick) {
	for _, p := range b.Points {
		stack[p.X][p.Y][p.Z] = true
	}
}

func (stack Stack) mayFall(b Brick) bool {
	if b.Points[0].Z == 1 {
		// on the ground
		return false
	}
	if b.Points[0].Z != b.Points[len(b.Points)-1].Z {
		// vertical
		bottom := b.Points[0]
		return !stack[bottom.X][bottom.Y][bottom.Z-1]
	}
	// horizontal
	return collections.All(b.Points, func(p Point) bool {
		return !stack[p.X][p.Y][p.Z-1]
	})
}

func parsePoint(str string) Point {
	coords := collections.ParseInts(strings.Split(str, ","))
	return Point{coords[0], coords[1], coords[2]}
}

func createBrick(from, to Point, id int) Brick {
	points := []Point{}
	if from.X != to.X {
		for x := from.X; x <= to.X; x++ {
			points = append(points, Point{x, from.Y, from.Z})
		}
	} else if from.Y != to.Y {
		for y := from.Y; y <= to.Y; y++ {
			points = append(points, Point{from.X, y, from.Z})
		}
	} else {
		for z := from.Z; z <= to.Z; z++ {
			points = append(points, Point{from.X, from.Y, z})
		}
	}
	return Brick{id, points}
}

func initStack(bricks []Brick, xSize, ySize, zSize int) Stack {
	stack := make([][][]bool, xSize)
	for x := 0; x < len(stack); x++ {
		stack[x] = make([][]bool, ySize)
		for y := 0; y < len(stack[x]); y++ {
			stack[x][y] = make([]bool, zSize)
		}
	}
	for _, b := range bricks {
		for _, pt := range b.Points {
			stack[pt.X][pt.Y][pt.Z] = true
		}
	}
	return stack
}

func (stack Stack) settle(bricks []Brick) {
	sort.Slice(bricks, func(i, j int) bool {
		return bricks[i].Points[0].Z < bricks[j].Points[0].Z
	})

	for b := 0; b < len(bricks); b++ {
		for stack.mayFall(bricks[b]) {
			stack.remove(bricks[b])
			bricks[b] = bricks[b].fall()
			stack.put(bricks[b])
		}
	}
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	bricks := []Brick{}
	maxX := 0
	maxY := 0
	maxZ := 0
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), "~")
		from := parsePoint(split[0])
		to := parsePoint(split[1])
		bricks = append(bricks, createBrick(from, to, len(bricks)))

		maxX = max(maxX, to.X)
		maxY = max(maxY, to.Y)
		maxZ = max(maxZ, to.Z)
	}

	stack := initStack(bricks, maxX+1, maxY+1, maxZ+1)
	stack.settle(bricks)

	first := 0
	for _, brick := range bricks {
		stack.remove(brick)
		if !collections.Any(bricks, stack.mayFall) {
			first++
		}
		stack.put(brick)
	}
	fmt.Println("First:", first)

	second := 0
	for b := 0; b < len(bricks); b++ {
		stack.remove(bricks[b])
		removed := []Brick{bricks[b]}
		for i := b + 1; i < len(bricks); i++ {
			if stack.mayFall(bricks[i]) {
				stack.remove(bricks[i])
				removed = append(removed, bricks[i])
				second++
			}
		}

		for _, b := range removed {
			stack.put(b)
		}
	}
	fmt.Println("Second:", second)
}
