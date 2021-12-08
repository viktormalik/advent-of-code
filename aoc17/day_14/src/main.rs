extern crate hex;

use knothash::*;
use std::fs;

fn to_bits(n: u8) -> Vec<bool> {
    (0..8).rev().map(|b| n & (1 << b) != 0).collect()
}

fn erase_group(groups: &mut Vec<Vec<bool>>, r: usize, c: usize) {
    groups[r][c] = false;
    if r > 0 && groups[r - 1][c] {
        erase_group(groups, r - 1, c);
    }
    if r < 127 && groups[r + 1][c] {
        erase_group(groups, r + 1, c);
    }
    if c > 0 && groups[r][c - 1] {
        erase_group(groups, r, c - 1);
    }
    if c < 127 && groups[r][c + 1] {
        erase_group(groups, r, c + 1);
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut grid: Vec<Vec<bool>> = (0..128)
        .map(|n| {
            hex::decode(knot_hash(
                &(input.trim().to_string() + "-" + &n.to_string()),
            ))
            .unwrap()
            .iter()
            .map(|cell| to_bits(*cell))
            .flatten()
            .collect()
        })
        .collect();

    let ones: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&cell| *cell).count())
        .sum();
    println!("First: {}", ones);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut groups = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] {
                groups += 1;
                erase_group(&mut grid, i, j);
            }
        }
    }
    println!("Second: {}", groups);
}
