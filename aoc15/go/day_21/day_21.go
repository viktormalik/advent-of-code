package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"gonum.org/v1/gonum/stat/combin"
)

type Item struct {
	Cost, Damage, Armor int
}

type Character struct {
	Hp, Damage, Armor int
}

func attack(attacker, defender *Character) {
	defender.Hp -= max(attacker.Damage-defender.Armor, 1)
}

func fight(player, boss Character) bool {
	playerTurn := true
	for player.Hp > 0 && boss.Hp > 0 {
		if playerTurn {
			attack(&player, &boss)
		} else {
			attack(&boss, &player)
		}
		playerTurn = !playerTurn
	}
	return player.Hp > 0
}

func equip(player Character, items []Item) Character {
	result := player
	for _, i := range items {
		result.Damage += i.Damage
		result.Armor += i.Armor
	}
	return result
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	var bossHp, bossDamage, bossArmor int
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), ": ")
		val, _ := strconv.Atoi(split[1])
		switch split[0] {
		case "Hit Points":
			bossHp = val
		case "Damage":
			bossDamage = val
		case "Armor":
			bossArmor = val
		}
	}
	boss := Character{bossHp, bossDamage, bossArmor}
	player := Character{100, 0, 0}

	weapons := []Item{
		{8, 4, 0},
		{10, 5, 0},
		{25, 6, 0},
		{40, 7, 0},
		{74, 8, 0},
	}
	weaponOpts := combin.Combinations(len(weapons), 1)
	armor := []Item{
		{13, 0, 1},
		{31, 0, 2},
		{53, 0, 3},
		{75, 0, 4},
		{102, 0, 5},
	}
	armorOpts := combin.Combinations(len(armor), 1)
	armorOpts = append(armorOpts, make([]int, 0))
	rings := []Item{
		{25, 1, 0},
		{50, 2, 0},
		{100, 3, 0},
		{20, 0, 1},
		{40, 0, 2},
		{80, 0, 3},
	}
	ringOpts := combin.Combinations(len(rings), 2)
	ringOpts = append(ringOpts, combin.Combinations(len(rings), 1)...)
	ringOpts = append(ringOpts, make([]int, 0))

	first := 0
	second := 0
	for _, w := range weaponOpts {
		for _, a := range armorOpts {
			for _, rs := range ringOpts {
				items := []Item{weapons[w[0]]}
				if len(a) == 1 {
					items = append(items, armor[a[0]])
				}
				for _, r := range rs {
					items = append(items, rings[r])
				}
				cost := collections.SumFunc(items, func(i Item) int { return i.Cost })
				win := fight(equip(player, items), boss)
				if win && (first == 0 || cost < first) {
					first = cost
				} else if !win && cost > second {
					second = cost
				}
			}
		}
	}
	fmt.Println("First:", first)
	fmt.Println("Second:", second)
}
