use intcode::*;
use std::collections::HashMap;
use std::fs;

fn paint(input: &String, start_color: i64) -> HashMap<(i32, i32), i64> {
    let mut prog = IntcodeProgram::load(&input);

    let mut panels = HashMap::new();
    let mut current = (0, 0);
    panels.insert(current, start_color);
    let mut dir = 0;

    while prog.state != ProgramState::Term {
        let color = panels.entry(current).or_insert(0);
        while prog.stdout.len() != 2 {
            prog.run(&vec![*color]);
        }

        *color = prog.stdout[0];
        dir =
            (dir + match prog.stdout[1] {
                0 => -1,
                1 => 1,
                _ => 0,
            }) % 4;
        if dir == -1 {
            dir = 3;
        }

        current.0 += match dir {
            1 => 1,
            3 => -1,
            _ => 0,
        };
        current.1 += match dir {
            0 => -1,
            2 => 1,
            _ => 0,
        };
        prog.stdout.clear();
    }
    panels
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let panels_black = paint(&input, 0);
    println!("First: {}", panels_black.len());

    let panels_white = paint(&input, 1);
    let minx = panels_white.keys().map(|(x, _)| x).min().unwrap();
    let maxx = panels_white.keys().map(|(x, _)| x).max().unwrap();
    let miny = panels_white.keys().map(|(_, y)| y).min().unwrap();
    let maxy = panels_white.keys().map(|(_, y)| y).max().unwrap();

    println!("Second:");
    for y in 0..maxy - miny + 1 {
        for x in 0..maxx - minx + 1 {
            print!(
                "{}",
                match panels_white.get(&(x + minx, y + miny)) {
                    Some(1) => '#',
                    _ => ' ',
                }
            );
        }
        println!();
    }
}
