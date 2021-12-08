use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn number(code: &str) -> u32 {
    code.chars()
        .zip((0..code.len()).rev())
        .fold(0, |num, (c, i)| {
            num + match c {
                'B' | 'R' => 2u32.pow(i as u32),
                _ => 0,
            }
        })
}

fn id(r: u32, c: u32) -> u32 {
    r * 8 + c
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let seats: HashSet<(u32, u32)> = input
        .lines()
        .map(|l| (number(&l[0..7]), number(&l[7..10])))
        .collect();

    let first = seats.iter().map(|&(r, c)| id(r, c)).max().unwrap();
    println!("First: {}", first);

    let seat = (0..128)
        .cartesian_product(0..8)
        .find(|&(r, c)| {
            !seats.contains(&(r, c)) && seats.contains(&(r, c + 1)) && seats.contains(&(r, c - 1))
        })
        .unwrap();
    println!("Second: {}", id(seat.0, seat.1));
}
