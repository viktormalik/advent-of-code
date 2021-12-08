use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let first = input
        .lines()
        .fold(0, |acc, line| acc + line.trim().parse::<i32>().unwrap());
    println!("First: {}", first);

    let mut result = 0;
    let mut seen = HashSet::new();
    loop {
        for line in input.lines() {
            result += line.parse::<i32>().unwrap();
            if seen.contains(&result) {
                println!("Second: {}", result);
                return;
            }
            seen.insert(result);
        }
    }
}
