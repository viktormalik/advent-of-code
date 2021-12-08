use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut insts: Vec<i32> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut ip = 0;
    let mut i = 0;
    while ip < insts.len() {
        let offset = insts[ip];
        insts[ip] += 1;
        ip = (ip as i32 + offset) as usize;
        i += 1;
    }
    println!("First: {}", i);

    insts = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    ip = 0;
    i = 0;
    while ip < insts.len() {
        let offset = insts[ip];
        insts[ip] += match offset >= 3 {
            true => -1,
            false => 1,
        };
        ip = (ip as i32 + offset) as usize;
        i += 1;
    }
    println!("Second: {}", i);
}
