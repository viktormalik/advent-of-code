use intcode::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    // Jump if you must (NOT A) or if you jump right behind a hole (NOT C) and there is no hole in
    // the destination (NOT NOT D).
    let mut prog = IntcodeProgram::load(&input);
    let springscript = "NOT A J\n\
                        NOT C T\n\
                        OR T J\n\
                        NOT D T\n\
                        NOT T T\n\
                        AND T J\n\
                        WALK\n";
    prog.run_ascii(&springscript);
    if prog.last_stdout() > 255 {
        println!("First: {}", prog.last_stdout());
    } else {
        println!("{}", prog.stdout_as_ascii());
    }

    let mut prog = IntcodeProgram::load(&input);
    let springscript = "NOT C J\n\
                        AND D J\n\
                        AND E J\n\
                        NOT C T\n\
                        AND D T\n\
                        AND H T\n\
                        OR T J\n\
                        NOT B T\n\
                        AND D T\n\
                        AND H T\n\
                        OR T J\n\
                        NOT A T\n\
                        OR T J\n\
                        RUN\n";
    prog.run_ascii(&springscript);
    if prog.last_stdout() > 255 {
        println!("Second: {}", prog.last_stdout());
    } else {
        println!("{}", prog.stdout_as_ascii());
    }
}
