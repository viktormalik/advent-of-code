package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func distances(time int) (result []int) {
	for hold := 0; hold <= time; hold++ {
		result = append(result, (time-hold)*hold)
	}
	return
}

func countWins(raceTime, raceDst int) int {
	return len(collections.Filter(distances(raceTime), func(dst int) bool {
		return dst > raceDst
	}))
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)
	scanner.Scan()
	timesStr := scanner.Text()
	scanner.Scan()
	dstsStr := scanner.Text()

	times := collections.ParseInts(strings.Fields(timesStr)[1:])
	dsts := collections.ParseInts(strings.Fields(dstsStr)[1:])
	first := 1
	for i := 0; i < len(times); i++ {
		first *= countWins(times[i], dsts[i])
	}
	fmt.Println("First:", first)

	totalTime, _ := strconv.Atoi(strings.ReplaceAll(timesStr[5:], " ", ""))
	totalDst, _ := strconv.Atoi(strings.ReplaceAll(dstsStr[9:], " ", ""))
	second := countWins(totalTime, totalDst)
	fmt.Println("Second:", second)
}
