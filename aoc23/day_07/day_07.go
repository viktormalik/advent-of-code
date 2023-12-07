package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const (
	HighCard  int = 0
	OnePair       = 1
	TwoPairs      = 2
	Three         = 3
	FullHouse     = 4
	Four          = 5
	Five          = 6
)

var cardValue = map[byte]int{
	'2': 2,
	'3': 3,
	'4': 4,
	'5': 5,
	'6': 6,
	'7': 7,
	'8': 8,
	'9': 9,
	'T': 10,
	'J': 11,
	'Q': 12,
	'K': 13,
	'A': 14,
}

type Hand struct {
	Cards string
	Bid   int
}

func countCard(cards string, c rune) int {
	return strings.Count(cards, string(c))
}

func countUnique(cards string) int {
	unique := []rune{}
	for _, c := range cards {
		unique = collections.SetAppend(unique, c)
	}
	return len(unique)
}

func hasCount(cards string, n int) bool {
	return collections.Any([]rune(cards), func(c rune) bool {
		return strings.Count(cards, string(c)) == n
	})
}

func kind(cards string) int {
	switch {
	case hasCount(cards, 5):
		return Five
	case hasCount(cards, 4):
		return Four
	case hasCount(cards, 3) && hasCount(cards, 2):
		return FullHouse
	case hasCount(cards, 3):
		return Three
	case hasCount(cards, 2) && countUnique(cards) == 3:
		return TwoPairs
	case hasCount(cards, 2):
		return OnePair
	default:
		return HighCard
	}
}

func kindJoker(cards string) int {
	switch kind(cards) {
	case Five:
		return Five
	case Four:
		if countCard(cards, 'J') >= 1 {
			return Five
		} else {
			return Four
		}
	case FullHouse:
		if countCard(cards, 'J') >= 1 {
			return Five
		} else {
			return FullHouse
		}
	case Three:
		if countCard(cards, 'J') >= 1 {
			return Four
		} else {
			return Three
		}
	case TwoPairs:
		if countCard(cards, 'J') == 2 {
			return Four
		} else if countCard(cards, 'J') == 1 {
			return FullHouse
		} else {
			return TwoPairs
		}
	case OnePair:
		if countCard(cards, 'J') >= 1 {
			return Three
		} else {
			return OnePair
		}
	default:
		if countCard(cards, 'J') == 1 {
			return OnePair
		} else {
			return HighCard
		}
	}
}

func lessCards(a, b string, kindFunc func(cards string) int) bool {
	ka := kindFunc(a)
	kb := kindFunc(b)
	switch {
	case ka < kb:
		return true
	case ka > kb:
		return false
	default:
		for i := 0; i < len(a); i++ {
			if a[i] != b[i] {
				return cardValue[a[i]] < cardValue[b[i]]
			}
		}
	}
	return true
}

func winnings(hands []Hand, kindFunc func(cards string) int) (result int) {
	sort.Slice(hands, func(i, j int) bool {
		return lessCards(hands[i].Cards, hands[j].Cards, kindFunc)
	})
	for rank := 0; rank < len(hands); rank++ {
		result += (rank + 1) * hands[rank].Bid
	}
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	hands := []Hand{}
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		bid, _ := strconv.Atoi(fields[1])
		hands = append(hands, Hand{fields[0], bid})
	}

	fmt.Println("First:", winnings(hands, kind))
	cardValue['J'] = 1
	fmt.Println("Second:", winnings(hands, kindJoker))
}
