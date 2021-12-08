extern crate primes;

use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq)]
enum State {
    RUN,
    TERM,
}

struct Prog {
    state: State,
    regs: HashMap<char, i64>,
    ip: usize,
}

impl Prog {
    fn run(&mut self, insts: &Vec<Vec<&str>>) -> i64 {
        let mut ret = 0;
        if self.state == State::RUN {
            let inst = &insts[self.ip];
            let mut new_ip = self.ip as i64 + 1;
            match inst[0] {
                "set" => *self.reg(inst[1]) = self.val(inst[2]),
                "sub" => *self.reg(inst[1]) -= self.val(inst[2]),
                "mul" => {
                    *self.reg(inst[1]) *= self.val(inst[2]);
                    ret = 1;
                }
                "jnz" => {
                    if self.val(inst[1]) != 0 {
                        new_ip = self.ip as i64 + self.val(inst[2]);
                    }
                }
                _ => {}
            }
            if new_ip >= insts.len() as i64 || new_ip < 0 {
                self.state = State::TERM;
            } else {
                self.ip = new_ip as usize;
            }
        }
        ret
    }

    fn val(&mut self, v: &str) -> i64 {
        match v.parse() {
            Ok(n) => n,
            Err(_) => *self.reg(v),
        }
    }

    fn reg(&mut self, r: &str) -> &mut i64 {
        self.regs.entry(r.chars().nth(0).unwrap()).or_insert(0)
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let insts: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut p = Prog {
        state: State::RUN,
        regs: ('a'..'i').map(|x| (x, 0)).collect(),
        ip: 0,
    };

    let mut first = 0;
    while p.state != State::TERM {
        if p.run(&insts) == 1 {
            first += 1;
        }
    }
    println!("First: {}", first);

    let second = (105700..122701)
        .step_by(17)
        .filter(|&n| !primes::is_prime(n))
        .count();
    println!("Second: {}", second);
}
