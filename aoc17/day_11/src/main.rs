use std::cmp;
use std::fs;

#[derive(Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn step(pos: &Pos, dir: &str) -> Pos {
    match dir {
        "n" => Pos {
            x: pos.x,
            y: pos.y + 1,
        },
        "s" => Pos {
            x: pos.x,
            y: pos.y - 1,
        },
        "ne" => Pos {
            x: pos.x + 1,
            y: match pos.x % 2 == 0 {
                true => pos.y + 1,
                false => pos.y,
            },
        },
        "nw" => Pos {
            x: pos.x - 1,
            y: match pos.x % 2 == 0 {
                true => pos.y + 1,
                false => pos.y,
            },
        },
        "se" => Pos {
            x: pos.x + 1,
            y: match pos.x % 2 == 0 {
                true => pos.y,
                false => pos.y - 1,
            },
        },
        "sw" => Pos {
            x: pos.x - 1,
            y: match pos.x % 2 == 0 {
                true => pos.y,
                false => pos.y - 1,
            },
        },
        _ => *pos,
    }
}

fn dst(pos: &Pos) -> i32 {
    pos.x.abs() + pos.y.abs() - (pos.x.abs() / 2)
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let end = input
        .trim()
        .split(',')
        .fold((Pos { x: 0, y: 0 }, 0), |state, dir| {
            (step(&state.0, dir), cmp::max(state.1, dst(&state.0)))
        });

    println!("First: {}", dst(&end.0));
    println!("Second: {}", end.1);
}
