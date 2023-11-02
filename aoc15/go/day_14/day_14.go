package main

import (
	"bufio"
	"cmp"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Reindeer struct {
	Speed, Fly, Rest, Dst, Pts int
}

func cmpDst(a, b *Reindeer) int {
	return cmp.Compare(a.Dst, b.Dst)
}

func cmpPts(a, b *Reindeer) int {
	return cmp.Compare(a.Pts, b.Pts)
}

func parse(line string) *Reindeer {
	split := strings.Fields(line)
	speed, _ := strconv.Atoi(split[3])
	fly, _ := strconv.Atoi(split[6])
	rest, _ := strconv.Atoi(split[13])
	return &Reindeer{speed, fly, rest, 0, 0}
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	reindeer := []*Reindeer{}
	for scanner.Scan() {
		reindeer = append(reindeer, parse(scanner.Text()))
	}

	for sec := 0; sec < 2503; sec++ {
		for _, r := range reindeer {
			if sec%(r.Fly+r.Rest) < r.Fly {
				r.Dst += r.Speed
			}
		}
		leaderDst := slices.MaxFunc(reindeer, cmpDst).Dst
		for _, r := range reindeer {
			if r.Dst == leaderDst {
				r.Pts++
			}
		}
	}

	fmt.Println("First:", slices.MaxFunc(reindeer, cmpDst).Dst)
	fmt.Println("Second:", slices.MaxFunc(reindeer, cmpPts).Pts)
}
