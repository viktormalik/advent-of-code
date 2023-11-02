package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var first int = 10000
var second int = 10000

type Character struct {
	Hp, Damage, Armor, Mana int
}

type Spell struct {
	Cost, Hp, Damage, Armor, Mana, Duration int
}

type Effect struct {
	Damage, Armor, Mana, Timer int
}

func (e Effect) from(s Spell) bool {
	return e.Damage == s.Damage && e.Armor == s.Armor && e.Mana == s.Mana
}

func attack(attacker, defender *Character) {
	defender.Hp -= max(attacker.Damage-defender.Armor, 1)
}

func cast(spell Spell, player, boss *Character, effects []Effect) []Effect {
	if spell.Duration == 0 {
		player.Hp += spell.Hp
		boss.Hp -= spell.Damage
		return effects
	} else {
		e := Effect{spell.Damage, spell.Armor, spell.Mana, spell.Duration}
		return append(effects, e)
	}
}

func apply(effects []Effect, player, boss *Character) {
	for i, e := range effects {
		boss.Hp -= e.Damage
		player.Armor += e.Armor
		player.Mana += e.Mana
		effects[i].Timer--
	}
}

func clear(effects []Effect, player *Character) (result []Effect) {
	for _, e := range effects {
		if e.Armor >= 0 {
			player.Armor -= e.Armor
		}
		if e.Timer > 0 {
			result = append(result, e)
		}
	}
	return
}

func applicable(spell Spell, effects []Effect) bool {
	return !collections.Any(effects, func(e Effect) bool {
		return e.Timer > 1 && e.from(spell)
	})
}

func win(cost int, best *int) {
	if cost < *best {
		*best = cost
	}
}

func fight(
	player, boss Character,
	spells []Spell,
	hard bool,
	best *int,
	effects []Effect,
	cost int,
	next Spell,
) {
	if cost >= *best {
		return
	}

	// Player turn

	// hard variant - loose a life
	if hard {
		player.Hp--
	}

	// apply effects
	apply(effects, &player, &boss)
	if boss.Hp <= 0 {
		win(cost, best)
		return
	}
	effects = clear(effects, &player)

	// cast new spell
	if player.Mana < next.Cost {
		return
	}
	player.Mana -= next.Cost
	effects = cast(next, &player, &boss, effects)
	cost += next.Cost
	if boss.Hp <= 0 {
		win(cost, best)
		return
	}

	// Boss turn

	// apply effects
	apply(effects, &player, &boss)
	if boss.Hp <= 0 {
		win(cost, best)
		return
	}

	// boss attack
	attack(&boss, &player)
	if player.Hp <= 0 {
		return
	}

	// clear effects
	effects = clear(effects, &player)

	// next round
	for _, spell := range spells {
		if applicable(spell, effects) {
			roundEffects := make([]Effect, len(effects))
			copy(roundEffects, effects)
			fight(player, boss, spells, hard, best, roundEffects, cost, spell)
		}
	}
}

func findBest(player, boss Character, spells []Spell, hard bool, best *int) {
	for _, s := range spells {
		fight(player, boss, spells, hard, best, []Effect{}, 0, s)
	}
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	var bossHp, bossDamage int
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), ": ")
		val, _ := strconv.Atoi(split[1])
		switch split[0] {
		case "Hit Points":
			bossHp = val
		case "Damage":
			bossDamage = val
		}
	}
	boss := Character{bossHp, bossDamage, 0, 0}
	player := Character{50, 0, 0, 500}

	spells := []Spell{
		{53, 0, 4, 0, 0, 0},
		{73, 2, 2, 0, 0, 0},
		{113, 0, 0, 7, 0, 6},
		{173, 0, 3, 0, 0, 6},
		{229, 0, 0, 0, 101, 5},
	}

	findBest(player, boss, spells, false, &first)
	fmt.Println("First:", first)
	findBest(player, boss, spells, true, &second)
	fmt.Println("Second:", second)
}
