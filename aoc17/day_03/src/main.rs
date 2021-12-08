use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq)]
enum Dir {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

fn next_dir(dir: &Dir) -> Dir {
    match dir {
        Dir::RIGHT => Dir::UP,
        Dir::UP => Dir::LEFT,
        Dir::LEFT => Dir::DOWN,
        Dir::DOWN => Dir::RIGHT,
    }
}

fn step(sq: (i32, i32), dir: &Dir) -> (i32, i32) {
    match dir {
        Dir::RIGHT => (sq.0 + 1, sq.1),
        Dir::UP => (sq.0, sq.1 + 1),
        Dir::LEFT => (sq.0 - 1, sq.1),
        Dir::DOWN => (sq.0, sq.1 - 1),
    }
}

fn sum_adj(sq: (i32, i32), grid: &HashMap<(i32, i32), u32>) -> u32 {
    (-1..2)
        .cartesian_product(-1..2)
        .map(|(x, y)| match grid.get(&(sq.0 + x, sq.1 + y)) {
            Some(&v) => v,
            None => 0,
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let target = input.trim().parse::<u32>().unwrap();

    let mut level = 0;
    let mut sq = (0, 0);
    let mut dir = Dir::RIGHT;
    let mut grid = HashMap::new();

    grid.insert(sq, 1);

    let mut second = 0;
    for _ in 1..target {
        let next = step(sq, &dir);

        if next.0.abs() > level || next.1.abs() > level {
            dir = next_dir(&dir);
            if dir == Dir::UP {
                level += 1;
                sq = next;
            } else {
                sq = step(sq, &dir);
            }
        } else {
            sq = next;
        }

        if second == 0 {
            let val = sum_adj(sq, &grid);
            if val < target {
                grid.insert(sq, val);
            } else {
                second = val;
            }
        }
    }

    println!("First: {}", sq.0.abs() + sq.1.abs());
    println!("Second: {}", second);
}
