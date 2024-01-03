package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"slices"
)

type Pos struct {
	Row, Col int
}

type Segment struct {
	Dest Pos
	Len  int
}

type Path struct {
	Crosses []Pos
	Len     int
}

type Grid [][]byte

func fourNeighs(pos Pos) []Pos {
	return []Pos{
		{pos.Row + 1, pos.Col},
		{pos.Row - 1, pos.Col},
		{pos.Row, pos.Col - 1},
		{pos.Row, pos.Col + 1},
	}
}

func neighsSlopes(pos Pos, grid Grid) []Pos {
	result := []Pos{}
	switch grid[pos.Row][pos.Col] {
	case '>':
		result = append(result, Pos{pos.Row, pos.Col + 1})
	case '<':
		result = append(result, Pos{pos.Row, pos.Col - 1})
	case 'v':
		result = append(result, Pos{pos.Row + 1, pos.Col})
	case '^':
		result = append(result, Pos{pos.Row - 1, pos.Col})
	default:
		result = fourNeighs(pos)
	}
	return collections.Filter(result, func(p Pos) bool {
		return p.Row >= 0 && p.Row < len(grid) &&
			p.Col >= 0 && p.Col < len(grid[p.Row]) &&
			(grid[p.Row][p.Col] == '.' ||
				grid[p.Row][p.Col] == '>' && p.Col == pos.Col+1 ||
				grid[p.Row][p.Col] == '<' && p.Col == pos.Col-1 ||
				grid[p.Row][p.Col] == 'v' && p.Row == pos.Row+1 ||
				grid[p.Row][p.Col] == '^' && p.Row == pos.Row-1)
	})
}

func neighsAll(pos Pos, grid Grid) []Pos {
	return collections.Filter(fourNeighs(pos), func(p Pos) bool {
		return p.Row >= 0 && p.Row < len(grid) &&
			p.Col >= 0 && p.Col < len(grid[p.Row]) &&
			grid[p.Row][p.Col] != '#'
	})
}

func getSegment(from, prev Pos, grid Grid, neighs func(Pos, Grid) []Pos) Segment {
	result := Segment{from, 0}
	for {
		ns := collections.Filter(neighs(result.Dest, grid), func(n Pos) bool {
			return n != prev
		})
		if len(ns) != 1 {
			break
		}
		result.Len++
		prev = result.Dest
		result.Dest = ns[0]
	}
	return result
}

func getSegments(start, end Pos, grid Grid, neighs func(Pos, Grid) []Pos) map[Pos][]Segment {
	todo := []Pos{start}
	result := map[Pos][]Segment{}
	for len(todo) > 0 {
		cur := todo[0]
		todo = todo[1:]
		for _, n := range neighs(cur, grid) {
			segment := getSegment(n, cur, grid, neighs)
			if segment.Len == 0 {
				continue
			}
			if _, ok := result[segment.Dest]; !ok {
				todo = append(todo, segment.Dest)
			}
			result[cur] = collections.SetAppend(result[cur], segment)
		}
	}
	return result
}

func allPaths(start, end Pos, segments map[Pos][]Segment) map[Pos]int {
	todo := []Path{{[]Pos{start}, 0}}
	result := map[Pos]int{}
	for len(todo) > 0 {
		cur := todo[0]
		todo = todo[1:]
		lastPos := cur.Crosses[len(cur.Crosses)-1]
		for _, seg := range segments[lastPos] {
			if slices.Contains(cur.Crosses, seg.Dest) {
				continue
			}

			newLen := cur.Len + seg.Len + 1
			if val, ok := result[seg.Dest]; ok && val >= newLen+700 {
				continue
			}

			result[seg.Dest] = max(result[seg.Dest], newLen)

			crosses := make([]Pos, len(cur.Crosses), len(cur.Crosses)+1)
			copy(crosses, cur.Crosses)
			crosses = append(crosses, seg.Dest)
			path := Path{crosses, newLen}
			if seg.Dest != end {
				todo = append(todo, path)
			}
		}
	}
	return result
}

func longest(start, end Pos, grid Grid, neighs func(Pos, Grid) []Pos) int {
	segments := getSegments(start, end, grid, neighs)
	paths := allPaths(start, end, segments)
	return paths[end]
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	grid := [][]byte{}
	for scanner.Scan() {
		grid = append(grid, []byte(scanner.Text()))
	}

	var start, end Pos
	for c := 0; c < len(grid[0]); c++ {
		if grid[0][c] == '.' {
			start = Pos{0, c}
		}
		if grid[len(grid)-1][c] == '.' {
			end = Pos{len(grid) - 1, c}
		}
	}

	fmt.Println("First:", longest(start, end, grid, neighsSlopes))
	fmt.Println("Second:", longest(start, end, grid, neighsAll))
}
