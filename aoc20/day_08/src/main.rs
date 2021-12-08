use std::collections::HashSet;
use std::fs;

#[derive(Eq, PartialEq)]
enum State {
    RUN,
    TERM,
    LOOP,
}

struct Prog {
    state: State,
    acc: i32,
    insts: Vec<Vec<String>>,
    ip: usize,
    executed: HashSet<usize>,
    flipped: Option<usize>,
}

impl Prog {
    fn run(&mut self) {
        while self.state == State::RUN {
            if self.executed.get(&self.ip).is_some() {
                self.state = State::LOOP;
                continue;
            } else {
                self.executed.insert(self.ip);
            }

            let inst = &self.insts[self.ip];
            let mut new_ip = self.ip + 1;
            match &inst[0][..] {
                "acc" => self.acc += inst[1].parse::<i32>().unwrap(),
                "jmp" => new_ip = (self.ip as i32 + inst[1].parse::<i32>().unwrap()) as usize,
                _ => {}
            }
            self.ip = new_ip;
            if self.ip == self.insts.len() {
                self.state = State::TERM
            }
        }
    }

    fn flip(&mut self, i: usize) {
        let inst = self.insts[i][0].clone();
        if inst == "nop" {
            self.insts[i][0] = "jmp".to_string();
        } else if inst == "jmp" {
            self.insts[i][0] = "nop".to_string();
        }
        self.flipped = Some(i);
    }

    fn reset(&mut self) {
        self.state = State::RUN;
        self.acc = 0;
        self.ip = 0;
        self.executed.clear();
        match self.flipped {
            Some(i) => self.flip(i),
            None => {}
        }
        self.flipped = None;
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut prog = Prog {
        state: State::RUN,
        acc: 0,
        insts: input
            .lines()
            .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
            .collect(),
        ip: 0,
        executed: HashSet::new(),
        flipped: None,
    };

    prog.run();
    println!("First: {}", prog.acc);
    prog.reset();

    for i in 0..prog.insts.len() {
        if prog.insts[i][0] == "acc" {
            continue;
        }
        prog.flip(i);
        prog.run();
        if prog.state == State::TERM {
            println!("Second: {}", prog.acc);
            return;
        }
        prog.reset();
    }
}
