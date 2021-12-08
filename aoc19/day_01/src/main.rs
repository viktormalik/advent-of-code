use std::fs;

fn fuel(weight: i32) -> i32 {
    match weight / 3 - 2 {
        -2..=0 => 0,
        f => f + fuel(f),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let total_basic: i32 = input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|m: i32| m / 3 - 2)
        .sum();
    println!("First: {}", total_basic);

    let total: i32 = input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|m: i32| fuel(m))
        .sum();
    println!("Second: {}", total);
}
