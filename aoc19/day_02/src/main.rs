use intcode::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut prog = IntcodeProgram::load(&input);
    prog.set_params(&vec![12, 2]);
    prog.run(&vec![]);
    println!("First: {}", prog.retval);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut prog = IntcodeProgram::load(&input);
            prog.set_params(&vec![noun, verb]);
            prog.run(&vec![]);
            if prog.retval == 19690720 {
                println!("Second: {}", 100 * noun + verb);
                return;
            }
        }
    }
}
