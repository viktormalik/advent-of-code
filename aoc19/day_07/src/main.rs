use intcode::*;
use permutohedron::Heap;
use std::fs;

fn get_thruster(input: &String, phase_seq: Vec<i64>) -> i64 {
    let mut programs: Vec<IntcodeProgram> = { 0..phase_seq.len() }
        .map(|_| IntcodeProgram::load(&input))
        .collect();
    let mut inputs: Vec<Vec<i64>> = phase_seq.iter().map(|&p| vec![p]).collect();
    inputs[0].push(0);

    let mut output = 0;
    while !programs.iter().all(|p| p.state == ProgramState::Term) {
        for i in 0..phase_seq.len() {
            programs[i].run(&inputs[i]);

            if i == 4 {
                output = programs[i].stdout[0];
            }

            inputs[i].clear();
            inputs[(i + 1) % phase_seq.len()].append(&mut programs[i].stdout);
        }
    }
    return output;
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let max_first = Heap::new(&mut vec![0, 1, 2, 3, 4])
        .map(|p| get_thruster(&input, p))
        .max()
        .unwrap();
    println!("First {}", max_first);

    let max_second = Heap::new(&mut vec![5, 6, 7, 8, 9])
        .map(|p| get_thruster(&input, p))
        .max()
        .unwrap();
    println!("Second: {}", max_second);
}
