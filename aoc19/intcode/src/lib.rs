#[derive(Eq, PartialEq)]
pub enum ProgramState {
    New,
    WaitInput,
    Term,
}

pub struct IntcodeProgram {
    program: Vec<i64>,
    ip: usize,
    rb: i64,
    pub state: ProgramState,
    pub stdout: Vec<i64>,
    pub retval: i64,
}

impl IntcodeProgram {
    pub fn load(prog: &String) -> IntcodeProgram {
        IntcodeProgram {
            program: prog.trim().split(",").map(|x| x.parse().unwrap()).collect(),
            ip: 0,
            rb: 0,
            state: ProgramState::New,
            stdout: vec![],
            retval: 0,
        }
    }

    pub fn set_params(&mut self, params: &Vec<i64>) {
        for i in 0..params.len() {
            self.program[i + 1] = params[i]
        }
    }

    pub fn set_mem(&mut self, index: usize, val: i64) {
        self.program[index] = val;
    }

    fn get_operand_index(&mut self, op: usize) -> usize {
        let op_idx = match self.program[self.ip] / (10 * 10i64.pow((op) as u32)) % 10 {
            0 => self.program[self.ip + op] as usize,
            1 => self.ip + op,
            2 => (self.rb + self.program[self.ip + op]) as usize,
            _ => op,
        };
        if op_idx >= self.program.len() {
            self.program.resize(op_idx + 1, 0);
        }
        op_idx
    }

    fn get_operand(&mut self, op: usize) -> i64 {
        let op_idx = self.get_operand_index(op);
        self.program[op_idx]
    }

    fn store(&mut self, op: usize, value: i64) {
        let result_idx = self.get_operand_index(op);
        self.program[result_idx] = value;
    }

    pub fn run(&mut self, input: &Vec<i64>) {
        let mut input_idx = 0;

        while self.program[self.ip] != 99 {
            let opcode = self.program[self.ip] % 100;

            match opcode {
                1 => {
                    // Addition
                    let val = self.get_operand(1) + self.get_operand(2);
                    self.store(3, val);
                    self.ip += 4;
                }
                2 => {
                    // Multiplication
                    let val = self.get_operand(1) * self.get_operand(2);
                    self.store(3, val);
                    self.ip += 4;
                }
                3 => {
                    // Read stdin
                    if input_idx < input.len() {
                        self.store(1, input[input_idx]);
                        input_idx += 1;
                        self.ip += 2;
                    } else {
                        self.state = ProgramState::WaitInput;
                        return;
                    }
                }
                4 => {
                    // Write stdout
                    let val = self.get_operand(1);
                    self.stdout.push(val);
                    self.ip += 2;
                }
                5 => {
                    // Jump if not zero
                    if self.get_operand(1) != 0 {
                        self.ip = self.get_operand(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    // Jump if zero
                    if self.get_operand(1) == 0 {
                        self.ip = self.get_operand(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    // Less than
                    let val = match self.get_operand(1) < self.get_operand(2) {
                        true => 1,
                        false => 0,
                    };
                    self.store(3, val);
                    self.ip += 4;
                }
                8 => {
                    // Equal
                    let val = match self.get_operand(1) == self.get_operand(2) {
                        true => 1,
                        false => 0,
                    };
                    self.store(3, val);
                    self.ip += 4;
                }
                9 => {
                    // Shift RB
                    self.rb += self.get_operand(1);
                    self.ip += 2;
                }
                _ => {}
            }
        }
        self.state = ProgramState::Term;
        self.retval = self.program[0];
    }

    pub fn run_ascii(&mut self, input: &str) {
        let input_numbers = input.chars().map(|c| c as u8 as i64).collect();
        self.run(&input_numbers)
    }

    pub fn stdout_as_ascii(&mut self) -> String {
        self.stdout.drain(..).map(|x| (x as u8) as char).collect()
    }

    pub fn last_stdout(&self) -> i64 {
        self.stdout[self.stdout.len() - 1]
    }
}
