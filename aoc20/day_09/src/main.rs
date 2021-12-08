use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let first = *numbers
        .iter()
        .enumerate()
        .skip(25)
        .find(|&(i, &num)| {
            numbers[i - 25..i]
                .iter()
                .cartesian_product(numbers[i - 25..i].iter())
                .find(|&(x, y)| x + y == num)
                .is_none()
        })
        .unwrap()
        .1;

    println!("First: {}", first);

    let mut start = 0;
    let mut end = 0;
    let mut sum = numbers[0];
    loop {
        if sum == first {
            println!(
                "Second: {}",
                numbers[start..=end].iter().min().unwrap()
                    + numbers[start..=end].iter().max().unwrap()
            );
            return;
        }

        end += 1;
        sum += numbers[end];

        while sum > first {
            sum -= numbers[start];
            start += 1;
        }
    }
}
