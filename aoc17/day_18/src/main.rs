use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

#[derive(Eq, PartialEq)]
enum State {
    RUN,
    WAIT,
    TERM,
}

struct Prog {
    state: State,
    regs: HashMap<char, i64>,
    ip: usize,
    queue: VecDeque<i64>,
}

impl Prog {
    fn run(&mut self, insts: &Vec<Vec<&str>>, other: &mut Prog) -> i64 {
        let mut ret = 0;
        if self.state == State::RUN {
            let inst = &insts[self.ip];
            let mut new_ip = self.ip as i64 + 1;
            match inst[0] {
                "set" => *self.reg(inst[1]) = self.val(inst[2]),
                "add" => *self.reg(inst[1]) += self.val(inst[2]),
                "mul" => *self.reg(inst[1]) *= self.val(inst[2]),
                "mod" => *self.reg(inst[1]) %= self.val(inst[2]),
                "snd" => other.queue.push_back(self.val(inst[1])),
                "rcv" => match self.queue.pop_front() {
                    Some(v) => {
                        let r = self.reg(inst[1]);
                        if *r != 0 {
                            ret = v;
                        }
                        *r = v;
                    }
                    None => self.state = State::WAIT,
                },
                "jgz" => {
                    if self.val(inst[1]) > 0 {
                        new_ip = self.ip as i64 + self.val(inst[2]);
                    }
                }
                _ => {}
            }
            if self.state == State::RUN {
                if new_ip >= insts.len() as i64 || new_ip < 0 {
                    self.state = State::TERM;
                } else {
                    self.ip = new_ip as usize;
                }
            }
        } else if self.state == State::WAIT {
            if !self.queue.is_empty() {
                self.state = State::RUN;
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

    let mut p1 = Prog {
        state: State::RUN,
        regs: HashMap::new(),
        ip: 0,
        queue: VecDeque::new(),
    };
    let mut p2 = Prog {
        state: State::RUN,
        regs: HashMap::new(),
        ip: 0,
        queue: VecDeque::new(),
    };

    loop {
        let res = p1.run(&insts, &mut p2);
        if res != 0 {
            println!("First: {}", res);
            break;
        }
        match p2.queue.pop_front() {
            Some(v) => p1.queue.push_front(v),
            None => {}
        }
    }

    p1 = Prog {
        state: State::RUN,
        regs: HashMap::new(),
        ip: 0,
        queue: VecDeque::new(),
    };
    p1.regs.insert('p', 0);
    p2 = Prog {
        state: State::RUN,
        regs: HashMap::new(),
        ip: 0,
        queue: VecDeque::new(),
    };
    p2.regs.insert('p', 1);

    let mut second = 0;
    while p1.state == State::RUN || p2.state == State::RUN {
        p1.run(&insts, &mut p2);
        let old_size = p1.queue.len();
        p2.run(&insts, &mut p1);
        if p1.queue.len() != old_size {
            second += 1;
        }
    }
    println!("Second: {}", second);
}
