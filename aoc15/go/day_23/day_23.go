package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Inst struct {
	Opcode string
	Reg    byte
	Offset int
}

func parse(line string) Inst {
	split := strings.Fields(line)
	switch split[0] {
	case "hlf", "tpl", "inc":
		return Inst{Opcode: split[0], Reg: split[1][0]}
	case "jmp":
		offset, _ := strconv.Atoi(split[1])
		return Inst{Opcode: split[0], Offset: offset}
	case "jie", "jio":
		offset, _ := strconv.Atoi(split[2])
		return Inst{Opcode: split[0], Reg: split[1][0], Offset: offset}
	}
	return Inst{}
}

func exec(insts []Inst, init int) int {
	regs := map[byte]int{'a': init, 'b': 0}
	ip := 0
	for ip >= 0 && ip < len(insts) {
		i := insts[ip]
		ip++
		switch i.Opcode {
		case "hlf":
			regs[i.Reg] /= 2
		case "tpl":
			regs[i.Reg] *= 3
		case "inc":
			regs[i.Reg] += 1
		case "jmp":
			ip += i.Offset - 1
		case "jie":
			if regs[i.Reg]%2 == 0 {
				ip += i.Offset - 1
			}
		case "jio":
			if regs[i.Reg] == 1 {
				ip += i.Offset - 1
			}
		}
	}
	return regs['b']
}

func main() {
	input, _ := os.Open("input")
	scanner := bufio.NewScanner(input)

	insts := []Inst{}
	for scanner.Scan() {
		insts = append(insts, parse(scanner.Text()))
	}

	fmt.Println("First:", exec(insts, 0))
	fmt.Println("Second:", exec(insts, 1))
}
