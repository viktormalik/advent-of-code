package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse(coords string) (x, y int) {
	split := strings.Split(coords, ",")
	x, _ = strconv.Atoi(split[0])
	y, _ = strconv.Atoi(split[1])
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	first := [1000 * 1000]int{}
	second := [1000 * 1000]int{}
	for scanner.Scan() {
		split := strings.Fields(scanner.Text())
		xs, ys := parse(split[len(split)-3])
		xe, ye := parse(split[len(split)-1])
		for x := xs; x <= xe; x++ {
			for y := ys; y <= ye; y++ {
				switch split[0] {
				case "turn":
					switch split[1] {
					case "on":
						first[x*1000+y] = 1
						second[x*1000+y]++
					case "off":
						first[x*1000+y] = 0
						if second[x*1000+y] > 0 {
							second[x*1000+y]--
						}
					}
				case "toggle":
					first[x*1000+y] = 1 - first[x*1000+y]
					second[x*1000+y] += 2
				}
			}
		}
	}
	fmt.Println("First:", collections.Sum(first[:]))
	fmt.Println("Second:", collections.Sum(second[:]))
}
