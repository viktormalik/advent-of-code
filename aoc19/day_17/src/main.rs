use intcode::*;
use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut prog = IntcodeProgram::load(&input);
    prog.run(&vec![]);

    let map_str = prog.stdout_as_ascii();
    let map: Vec<Vec<char>> = map_str
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect())
        .collect();

    let alignment: usize = { 1..map.len() - 1 }
        .cartesian_product(1..map[0].len() - 1)
        .filter(|&(x, y)| {
            map[x][y] == '#'
                && map[x - 1][y] == '#'
                && map[x + 1][y] == '#'
                && map[x][y - 1] == '#'
                && map[x][y + 1] == '#'
        })
        .map(|(x, y)| x * y)
        .sum();
    println!("First: {}", alignment);

    let mut prog = IntcodeProgram::load(&input);
    prog.set_mem(0, 2);

    prog.run_ascii("A,B,A,B,C,C,B,A,B,C\n");
    prog.run_ascii("L,8,R,12,R,12,R,10\n");
    prog.run_ascii("R,10,R,12,R,10\n");
    prog.run_ascii("L,10,R,10,L,6\n");
    prog.run_ascii("n\n");

    println!("Second: {}", prog.last_stdout());
}
