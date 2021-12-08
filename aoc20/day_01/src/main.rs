use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let first = nums
        .iter()
        .cartesian_product(nums.iter())
        .find(|(&a, &b)| a + b == 2020)
        .unwrap();
    println!("First: {}", first.0 * first.1);

    let second = nums
        .iter()
        .cartesian_product(nums.iter())
        .cartesian_product(nums.iter())
        .find(|((&a, &b), &c)| a + b + c == 2020)
        .unwrap();
    println!("Second: {}", second.0 .0 * second.0 .1 * second.1);
}
