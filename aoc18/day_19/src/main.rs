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

fn run(init_regs: &Vec<usize>, instructions: &Vec<Inst>, ip_reg: usize) -> usize {
    let mut regs = init_regs.to_vec();
    loop {
        if regs[ip_reg] >= instructions.len() {
            break;
        }

        if regs[3] == 9 && regs[1] < regs[2] / regs[4] {
            while (regs[2] % regs[4]) != 0 {
                regs[4] += 1;
            }
            regs[1] = regs[2] / regs[4];
        }

        let prev_4 = regs[4];
        exec(&instructions[regs[ip_reg]], &mut regs);
        if regs[4] != prev_4 && regs[4] < regs[2] {
            while (regs[2] % regs[4]) != 0 {
                regs[4] += 1;
            }
        }
        regs[ip_reg] += 1;
    }
    regs[0]
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

    let first = run(&vec![0; 6], &instructions, ip_reg);
    println!("First: {}", first);

    let mut regs: Vec<usize> = vec![0; 6];
    regs[0] = 1;
    let second = run(&regs, &instructions, ip_reg);
    println!("Second: {}", second);
}
