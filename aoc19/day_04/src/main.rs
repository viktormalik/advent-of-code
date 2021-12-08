extern crate itertools;
use itertools::Itertools;
use std::fs;

fn check_first(num: usize) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    return !digits.iter().tuple_windows().any(|(x, y)| x > y)
        && digits.iter().tuple_windows().any(|(x, y)| x == y);
}

fn check_second(num: usize) -> bool {
    let mut prev = 0;
    let mut rep = 1;
    let mut has_double = false;
    for c in num.to_string().chars() {
        let d = c.to_digit(10).unwrap();
        if d == prev {
            rep += 1;
        } else if d > prev {
            if rep == 2 {
                has_double = true;
            }
            rep = 1;
        } else {
            return false;
        }
        prev = d;
    }
    return has_double || rep == 2;
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let a: usize = input.trim().split("-").nth(0).unwrap().parse().unwrap();
    let b: usize = input.trim().split("-").nth(1).unwrap().parse().unwrap();

    let count = { a..b + 1 }.filter(|&x| check_first(x)).count();
    println!("First: {}", count);
    let count = { a..b + 1 }.filter(|&x| check_second(x)).count();
    println!("Second: {}", count);
}
