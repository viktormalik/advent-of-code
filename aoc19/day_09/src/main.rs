use intcode::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut prog = IntcodeProgram::load(&input);
    prog.run(&vec![1]);
    println!("First: {:?}", prog.stdout);

    let mut prog = IntcodeProgram::load(&input);
    prog.run(&vec![2]);
    println!("Second: {:?}", prog.stdout);
}
