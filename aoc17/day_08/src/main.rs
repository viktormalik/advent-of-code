use std::cmp;
use std::collections::HashMap;
use std::fs;

enum Op {
    INC,
    DEC,
}

enum Rel {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}

struct Cond {
    reg: String,
    op: Rel,
    val: i32,
}

struct Inst {
    reg: String,
    op: Op,
    val: i32,
    cond: Cond,
}

fn parse(inst: &str) -> Inst {
    let parts: Vec<&str> = inst.split_whitespace().collect();
    Inst {
        reg: parts[0].to_string(),
        op: match parts[1] {
            "inc" => Op::INC,
            _ => Op::DEC,
        },
        val: parts[2].parse().unwrap(),
        cond: Cond {
            reg: parts[4].to_string(),
            op: match parts[5] {
                "!=" => Rel::NE,
                "<" => Rel::LT,
                "<=" => Rel::LE,
                ">" => Rel::GT,
                ">=" => Rel::GE,
                _ => Rel::EQ,
            },
            val: parts[6].parse().unwrap(),
        },
    }
}

fn eval(cond: &Cond, regs: &mut HashMap<String, i32>) -> bool {
    let v = *regs.entry(cond.reg.clone()).or_insert(0);
    match cond.op {
        Rel::EQ => v == cond.val,
        Rel::NE => v != cond.val,
        Rel::LT => v < cond.val,
        Rel::LE => v <= cond.val,
        Rel::GT => v > cond.val,
        Rel::GE => v >= cond.val,
    }
}

fn run(inst: &Inst, regs: &mut HashMap<String, i32>) {
    let cond = eval(&inst.cond, regs);
    let reg_val = regs.entry(inst.reg.clone()).or_insert(0);
    if cond {
        *reg_val = match inst.op {
            Op::INC => *reg_val + inst.val,
            Op::DEC => *reg_val - inst.val,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let insts: Vec<Inst> = input.lines().map(|l| parse(l)).collect();
    let mut regs = HashMap::new();

    let mut max = 0;
    for i in insts {
        run(&i, &mut regs);
        max = cmp::max(max, *regs.values().max().unwrap());
    }

    let first = regs.values().max().unwrap();
    println!("First: {}", first);
    println!("Second: {}", max);
}
