package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/TheAlgorithms/Go/math/lcm"
	"golang.org/x/exp/maps"
)

type Dirs struct {
	L, R string
}

func steps(node string, insts []byte, net map[string]Dirs, cond func(string) bool) (result int) {
	current := node
	i := 0
	for !cond(current) {
		switch insts[i] {
		case 'L':
			current = net[current].L
		case 'R':
			current = net[current].R
		}
		i = (i + 1) % len(insts)
		result++
	}
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	var insts []byte
	net := map[string]Dirs{}
	for scanner.Scan() {
		line := scanner.Text()
		if strings.Contains(line, "=") {
			node := strings.Split(line, " = ")
			dirs := strings.Split(node[1], ", ")
			net[node[0]] = Dirs{dirs[0][1:], dirs[1][:3]}
		} else if line != "" {
			insts = []byte(line)
		}
	}

	fmt.Println("First:", steps("AAA", insts, net, func(n string) bool {
		return n == "ZZZ"
	}))

	starts := collections.Filter(maps.Keys(net), func(n string) bool {
		return n[2] == 'A'
	})
	dsts := collections.Map(starts, func(s string) int64 {
		return int64(steps(s, insts, net, func(n string) bool {
			return n[2] == 'Z'
		}))
	})
	second := collections.Reduce(dsts, 1, lcm.Lcm)
	fmt.Println("Second:", second)
}
