use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let sheet: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let checksum: u32 = sheet
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum();
    println!("First: {}", checksum);

    let second: u32 = sheet
        .iter()
        .map(|row| {
            row.iter()
                .cartesian_product(row.iter())
                .find(|(&x, &y)| x != y && (x % y) == 0)
                .unwrap()
        })
        .map(|(x, y)| x / y)
        .sum();
    println!("Second: {:?}", second);
}
