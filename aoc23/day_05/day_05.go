package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Interval struct {
	Start, End int
}

type MapEntry struct {
	Src, Dest Interval
}

type Map struct {
	Entries []MapEntry
}

func (m *Map) add(srcStart, srcEnd, destStart, destEnd int) {
	m.Entries = append(m.Entries,
		MapEntry{Interval{srcStart, srcEnd}, Interval{destStart, destEnd}})
}

func convert(interval Interval, m Map) (result []Interval) {
	queue := []Interval{interval}
	for len(queue) > 0 {
		i := queue[0]
		queue = queue[1:]
		converted := false
		for _, e := range m.Entries {
			if i.End <= e.Src.Start || i.Start >= e.Src.End {
				continue
			}
			newStart := max(e.Dest.Start, e.Dest.Start+(i.Start-e.Src.Start))
			newEnd := min(e.Dest.End, e.Dest.End-(e.Src.End-i.End))
			result = append(result, Interval{newStart, newEnd})
			if i.Start < e.Src.Start {
				queue = append(queue, Interval{i.Start, e.Src.Start})
			}
			if i.End >= e.Src.End {
				queue = append(queue, Interval{e.Src.End, i.End})
			}
			converted = true
		}
		if !converted {
			// no map entry
			result = append(result, i)
		}
	}
	return
}

func propagate(interval Interval, maps []Map) (result []Interval) {
	result = append(result, interval)
	for _, m := range maps {
		converted := []Interval{}
		for _, i := range result {
			converted = append(converted, convert(i, m)...)
		}
		result = converted
	}
	return
}

func findMin(intervals []Interval, maps []Map) (result int) {
	for _, interval := range intervals {
		for _, i := range propagate(interval, maps) {
			if result == 0 || i.Start < result {
				result = i.Start
			}
		}
	}
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	firstSeeds := []Interval{}
	secondSeeds := []Interval{}
	maps := []Map{}
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "seeds:") {
			ns := strings.Fields(line[7:])
			for s := 0; s < len(ns); s += 2 {
				first, _ := strconv.Atoi(ns[s])
				second, _ := strconv.Atoi(ns[s+1])
				firstSeeds = append(firstSeeds, Interval{first, first + 1})
				firstSeeds = append(firstSeeds, Interval{second, second + 1})
				secondSeeds = append(secondSeeds, Interval{first, first + second})
			}
		} else if strings.Contains(line, "map:") {
			maps = append(maps, Map{[]MapEntry{}})
		} else if line != "" {
			nums := strings.Fields(line)
			dest, _ := strconv.Atoi(nums[0])
			src, _ := strconv.Atoi(nums[1])
			length, _ := strconv.Atoi(nums[2])
			maps[len(maps)-1].add(src, src+length, dest, dest+length)
		}
	}

	fmt.Println("First:", findMin(firstSeeds, maps))
	fmt.Println("Second:", findMin(secondSeeds, maps))
}
