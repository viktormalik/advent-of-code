extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Hash, Eq, PartialEq)]
enum Opcode {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

fn exec(opcode: &Opcode, args: &[usize], registers: &mut Vec<usize>) {
    registers[args[2]] = match opcode {
        Opcode::ADDR => registers[args[0]] + registers[args[1]],
        Opcode::ADDI => registers[args[0]] + args[1],
        Opcode::MULR => registers[args[0]] * registers[args[1]],
        Opcode::MULI => registers[args[0]] * args[1],
        Opcode::BANR => registers[args[0]] & registers[args[1]],
        Opcode::BANI => registers[args[0]] & args[1],
        Opcode::BORR => registers[args[0]] | registers[args[1]],
        Opcode::BORI => registers[args[0]] | args[1],
        Opcode::SETR => registers[args[0]],
        Opcode::SETI => args[0],
        Opcode::GTIR => (args[0] > registers[args[1]]) as usize,
        Opcode::GTRI => (registers[args[0]] > args[1]) as usize,
        Opcode::GTRR => (registers[args[0]] > registers[args[1]]) as usize,
        Opcode::EQIR => (args[0] == registers[args[1]]) as usize,
        Opcode::EQRI => (registers[args[0]] == args[1]) as usize,
        Opcode::EQRR => (registers[args[0]] == registers[args[1]]) as usize,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let parts: Vec<&str> = input.trim().split("\n\n\n").collect();

    let before_re = Regex::new(r"^Before: \[(.*)\]$").unwrap();
    let after_re = Regex::new(r"^After:  \[(.*)\]$").unwrap();

    let mut codes: Vec<HashSet<Opcode>> = vec![HashSet::new(); 16];

    let mut result = 0;
    for sample in parts[0].trim().split("\n\n") {
        let lines: Vec<&str> = sample.trim().lines().collect();
        let before: Vec<usize> = before_re.captures(lines[0]).unwrap()[1]
            .split(",")
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect();
        let inst: Vec<usize> = lines[1]
            .split(" ")
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect();
        let after: Vec<usize> = after_re.captures(lines[2]).unwrap()[1]
            .split(",")
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect();

        let mut cnt = 0;
        for opcode in [
            Opcode::ADDR,
            Opcode::ADDI,
            Opcode::MULR,
            Opcode::MULI,
            Opcode::BANR,
            Opcode::BANI,
            Opcode::BORR,
            Opcode::BORI,
            Opcode::SETR,
            Opcode::SETI,
            Opcode::GTIR,
            Opcode::GTRI,
            Opcode::GTRR,
            Opcode::EQIR,
            Opcode::EQRI,
            Opcode::EQRR,
        ]
        .iter()
        {
            let mut reg = before.clone();
            exec(opcode, &inst[1..], &mut reg);
            if reg == after {
                cnt += 1;
                codes[inst[0]].insert(opcode.clone());
            }
        }
        if cnt >= 3 {
            result += 1;
        }
    }
    println!("First: {}", result);

    while codes.iter().any(|c| c.len() > 1) {
        for i in 0usize..codes.len() {
            if codes[i].len() == 1 {
                let code = codes[i].iter().next().unwrap().clone();
                for j in 0usize..codes.len() {
                    if j != i {
                        codes[j].remove(&code);
                    }
                }
            }
        }
    }

    let mut registers: Vec<usize> = vec![0; 4];
    for command in parts[1].trim().lines().map::<Vec<usize>, _>(|line| {
        line.trim()
            .split(" ")
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect()
    }) {
        exec(
            codes[command[0]].iter().next().unwrap(),
            &command[1..],
            &mut registers,
        );
    }
    println!("Second: {}", registers[0]);
}
