package main

import (
	"bufio"
	"fmt"
	"maps"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Rule struct {
	Prop, Op string
	Val      int
	Dest     string
}

type Part map[string]int

type Range struct {
	From, To int
}

type WfRange map[string]Range

func (r WfRange) SetFrom(prop string, val int) {
	propRange := r[prop]
	propRange.From = val
	r[prop] = propRange
}

func (r WfRange) SetTo(prop string, val int) {
	propRange := r[prop]
	propRange.To = val
	r[prop] = propRange
}

func parseWorkflow(line string) (name string, rules []Rule) {
	workflowRe := regexp.MustCompile("([a-z]+){(.*)}")
	ruleRe := regexp.MustCompile("([a-z])([<>])([0-9]+):([a-zAR]+)|[a-zAR]+")
	split := workflowRe.FindStringSubmatch(line)
	name = split[1]
	for _, rule := range ruleRe.FindAllStringSubmatch(split[2], -1) {
		if rule[1] == "" {
			rules = append(rules, Rule{"", "", 0, rule[0]})
		} else {
			val, _ := strconv.Atoi(rule[3])
			rules = append(rules, Rule{rule[1], rule[2], val, rule[4]})
		}
	}
	return
}

func parsePart(line string) Part {
	result := Part{}
	split := strings.Split(line[1:len(line)-1], ",")
	for _, prop := range split {
		val, _ := strconv.Atoi(prop[2:])
		result[prop[0:1]] = val
	}
	return result
}

func process(part Part, workflows map[string][]Rule) bool {
	cur := "in"
	for cur != "A" && cur != "R" {
	ruleloop:
		for _, rule := range workflows[cur] {
			switch rule.Op {
			case "":
				cur = rule.Dest
				break ruleloop
			case ">":
				if part[rule.Prop] > rule.Val {
					cur = rule.Dest
					break ruleloop
				}
			case "<":
				if part[rule.Prop] < rule.Val {
					cur = rule.Dest
					break ruleloop
				}
			}
		}
	}
	return cur == "A"
}

func rating(part Part) (result int) {
	for _, val := range part {
		result += val
	}
	return
}

func size(wr WfRange) int {
	result := 1
	for _, r := range wr {
		result *= r.To - r.From + 1
	}
	return result
}

func resolveRanges(ranges map[string]WfRange, workflows map[string][]Rule) (result int) {
	for len(ranges) > 0 {
		for name := range ranges {
			wfRange := WfRange{}
			maps.Copy(wfRange, ranges[name])
			for _, rule := range workflows[name] {
				ruleRange := WfRange{}
				maps.Copy(ruleRange, wfRange)
				switch rule.Op {
				case "<":
					ruleRange.SetTo(rule.Prop, rule.Val-1)
					wfRange.SetFrom(rule.Prop, rule.Val)
				case ">":
					ruleRange.SetFrom(rule.Prop, rule.Val+1)
					wfRange.SetTo(rule.Prop, rule.Val)
				}
				if rule.Dest == "A" {
					result += size(ruleRange)
				} else if rule.Dest != "R" {
					ranges[rule.Dest] = ruleRange
				}
			}
			delete(ranges, name)
		}
	}
	return
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	workflows := map[string][]Rule{}
	parts := []Part{}
	blank := false
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			blank = true
			continue
		}
		if !blank {
			name, rules := parseWorkflow(line)
			workflows[name] = rules
		} else {
			parts = append(parts, parsePart(line))
		}
	}

	first := 0
	for _, part := range parts {
		if process(part, workflows) {
			first += rating(part)
		}
	}
	fmt.Println("First:", first)

	ranges := map[string]WfRange{"in": {
		"x": Range{1, 4000},
		"m": Range{1, 4000},
		"a": Range{1, 4000},
		"s": Range{1, 4000},
	}}
	fmt.Println("Second:", resolveRanges(ranges, workflows))
}
