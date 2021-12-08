use std::fs;

struct Inst {
    op: String,
    args: Vec<usize>,
}

fn exec(instr: &Inst, registers: &mut Vec<usize>) {
    registers[instr.args[2]] = match instr.op.as_ref() {
        "addr" => registers[instr.args[0]] + registers[instr.args[1]],
        "addi" => registers[instr.args[0]] + instr.args[1],
        "mulr" => registers[instr.args[0]] * registers[instr.args[1]],
        "muli" => registers[instr.args[0]] * instr.args[1],
        "banr" => registers[instr.args[0]] & registers[instr.args[1]],
        "bani" => registers[instr.args[0]] & instr.args[1],
        "borr" => registers[instr.args[0]] | registers[instr.args[1]],
        "bori" => registers[instr.args[0]] | instr.args[1],
        "setr" => registers[instr.args[0]],
        "seti" => instr.args[0],
        "gtir" => (instr.args[0] > registers[instr.args[1]]) as usize,
        "gtri" => (registers[instr.args[0]] > instr.args[1]) as usize,
        "gtrr" => (registers[instr.args[0]] > registers[instr.args[1]]) as usize,
        "eqir" => (instr.args[0] == registers[instr.args[1]]) as usize,
        "eqri" => (registers[instr.args[0]] == instr.args[1]) as usize,
        "eqrr" => (registers[instr.args[0]] == registers[instr.args[1]]) as usize,
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let ip_reg = input
        .trim()
        .lines()
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let instructions: Vec<Inst> = input
        .trim()
        .lines()
        .skip(1)
        .map(|line| {
            let l: Vec<&str> = line.split(" ").collect();
            Inst {
                op: l[0].to_string(),
                args: l
                    .iter()
                    .skip(1)
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect();

    let mut regs: Vec<usize> = vec![0; 6];
    let mut reg1_hist: Vec<usize> = Vec::new();

    loop {
        if regs[ip_reg] >= instructions.len() {
            break;
        }

        exec(&instructions[regs[ip_reg]], &mut regs);
        regs[ip_reg] += 1;

        if regs[ip_reg] == 28 {
            if reg1_hist.is_empty() {
                println!("First: {}", regs[1]);
            }
            if reg1_hist.iter().any(|&r| r == regs[1]) {
                println!("Second: {}", reg1_hist.last().unwrap());
                break;
            }
            reg1_hist.push(regs[1]);
        }
    }
}
