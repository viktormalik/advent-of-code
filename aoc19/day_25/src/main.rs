extern crate regex;

use intcode::*;
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let mut prog = IntcodeProgram::load(&input);

    prog.run(&vec![]);
    prog.run_ascii("south\n");
    prog.run_ascii("take fuel cell\n");
    prog.run_ascii("north\n");
    prog.run_ascii("west\n");
    prog.run_ascii("take mutex\n");
    prog.run_ascii("south\n");
    prog.run_ascii("south\n");
    prog.run_ascii("take coin\n");
    prog.run_ascii("north\n");
    prog.run_ascii("east\n");
    prog.run_ascii("take cake\n");
    prog.run_ascii("north\n");
    prog.run_ascii("west\n");
    prog.run_ascii("south\n");
    prog.run_ascii("west\n");

    let output = prog.stdout_as_ascii();

    let re = Regex::new(r"(\d+)").unwrap();
    let pass = re.captures(&output).unwrap().get(1).unwrap().as_str();
    println!("First: {}", pass);
}
