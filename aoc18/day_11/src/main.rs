extern crate itertools;

use itertools::Itertools;
use std::fs;

fn total_power(x: usize, y: usize, size: usize, grid: &Vec<Vec<i32>>) -> i32 {
    grid[x..x + size]
        .iter()
        .map(|row| row[y..y + size].iter().sum::<i32>())
        .sum()
}

fn find_max_power(size: usize, grid: &Vec<Vec<i32>>) -> ((usize, usize), i32) {
    (0..grid.len() - size)
        .cartesian_product(0..grid[0].len() - size)
        .map(|pos| (pos, total_power(pos.0, pos.1, size, grid)))
        .max_by_key(|(_, power)| *power)
        .unwrap()
}

fn main() {
    let serial: i32 = fs::read_to_string("input")
        .expect("Error reading input")
        .trim()
        .parse()
        .unwrap();
    let grid: Vec<Vec<i32>> = { 0..300 }
        .map(|x| {
            { 0..300 }
                .map(|y| {
                    let id = (x as i32) + 1 + 10;
                    let power = (id * ((y as i32) + 1) + serial) * id;
                    ((power / 100) % 10) - 5
                })
                .collect()
        })
        .collect();

    let first = find_max_power(3, &grid);
    println!("First: {},{}", first.0 .0 + 1, first.0 .1 + 1);

    let second = (1..300)
        .map(|size| (find_max_power(size, &grid), size))
        .max_by_key(|((_, power), _)| *power)
        .unwrap();
    println!(
        "Second: {},{},{}",
        second.0 .0 .0 + 1,
        second.0 .0 .1 + 1,
        second.1
    );
}
