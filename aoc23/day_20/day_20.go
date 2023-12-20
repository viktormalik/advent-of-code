package main

import (
	"aoc/collections"
	"bufio"
	"fmt"
	"os"
	"strings"

	"golang.org/x/exp/maps"
)

const (
	FLIP  = 0
	CONJ  = 1
	BCAST = 2
)

type Module struct {
	Kind    int
	Inputs  []string
	Outputs []string
	State   bool
	InVals  map[string]bool
}

func (m Module) allInputsHigh() bool {
	return collections.All(maps.Values(m.InVals), func(v bool) bool {
		return v
	})
}

type Pulse struct {
	From, To string
	Value    bool
}

func send(name string, modules map[string]Module, val bool, pulses *[]Pulse) {
	for _, out := range modules[name].Outputs {
		*pulses = append(*pulses, Pulse{name, out, val})
	}
}

func press(modules map[string]Module) []Pulse {
	pulses := []Pulse{{"button", "broadcaster", false}}
	send("broadcaster", modules, false, &pulses)
	for s := 0; s < len(pulses); s++ {
		pulse := pulses[s]
		mod := modules[pulse.To]
		switch mod.Kind {
		case FLIP:
			if pulse.Value == false {
				mod.State = !mod.State
				send(pulse.To, modules, mod.State, &pulses)
				modules[pulse.To] = mod
			}
		case CONJ:
			modules[pulse.To].InVals[pulse.From] = pulse.Value
			if modules[pulse.To].allInputsHigh() {
				send(pulse.To, modules, false, &pulses)
			} else {
				send(pulse.To, modules, true, &pulses)
			}
		}
	}
	return pulses
}

func findPeriod(modules map[string]Module, module string, value bool) int {
	initModules(modules)
	period := 0
	last := 0
	for i := 0; true; i++ {
		pulses := press(modules)
		if collections.Any(pulses, func(pulse Pulse) bool {
			return pulse.From == module && pulse.Value == value
		}) {
			if i-last == period {
				return period
			} else {
				period = i - last
				last = i
			}
		}
	}
	return -1
}

func parseModule(line string) (name string, mod Module) {
	split := strings.Split(line, " -> ")
	outputs := strings.Split(split[1], ", ")
	if split[0] == "broadcaster" {
		name = split[0]
		mod = Module{BCAST, []string{}, outputs, false, nil}
	} else if split[0][0] == '%' {
		name = split[0][1:]
		mod = Module{FLIP, []string{}, outputs, false, nil}
	} else {
		name = split[0][1:]
		mod = Module{CONJ, []string{}, outputs, false, map[string]bool{}}
	}
	return
}

func setInputs(modules map[string]Module) {
	for name, mod := range modules {
		for _, out := range mod.Outputs {
			target := modules[out]
			target.Inputs = append(target.Inputs, name)
			modules[out] = target
		}
	}
}

func initModules(modules map[string]Module) {
	for name, mod := range modules {
		switch mod.Kind {
		case FLIP:
			mod.State = false
			modules[name] = mod
		case CONJ:
			for _, in := range mod.Inputs {
				mod.InVals[in] = false
			}
		}
	}
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	modules := map[string]Module{}
	for scanner.Scan() {
		name, mod := parseModule(scanner.Text())
		modules[name] = mod
	}
	setInputs(modules)

	initModules(modules)
	var low, high int
	for i := 0; i < 1000; i++ {
		pulses := press(modules)
		low += collections.CountFunc(pulses, func(s Pulse) bool {
			return s.Value == false
		})
		high += collections.CountFunc(pulses, func(s Pulse) bool {
			return s.Value == true
		})
	}
	fmt.Println("First:", low*high)

	targets := modules["rx"].Inputs
	targetVal := false
	for len(targets) == 1 {
		targets = modules[targets[0]].Inputs
		targetVal = !targetVal
	}
	second := 1
	for _, target := range targets {
		second *= findPeriod(modules, target, targetVal)
	}
	fmt.Println("Second:", second)
}
