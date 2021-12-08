use itertools::sorted;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let first = input
        .lines()
        .filter(|line| {
            !line.split_whitespace().any(|word| {
                line.split_whitespace()
                    .filter(|other| *other == word)
                    .count()
                    > 1
            })
        })
        .count();

    let second = input
        .lines()
        .filter(|line| {
            !line.split_whitespace().any(|word| {
                line.split_whitespace()
                    .filter(|other| sorted(other.chars()).eq(sorted(word.chars())))
                    .count()
                    > 1
            })
        })
        .count();

    println!("First: {}", first);
    println!("Second: {}", second);
}
