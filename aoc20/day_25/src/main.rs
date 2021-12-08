use std::fs;

const MAX: usize = 100000000;

fn find_loop_size(key: u64) -> u64 {
    let mut result = 1;
    for loop_size in 0..MAX {
        result = (result * 7) % 20201227;

        if result == key {
            return loop_size as u64 + 1;
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let card_key: u64 = input.lines().nth(0).unwrap().parse().unwrap();
    let door_key: u64 = input.lines().nth(1).unwrap().parse().unwrap();

    let card_loop_size = find_loop_size(card_key);
    let first = (0..card_loop_size).fold(1u64, |val, _| (val * door_key) % 20201227);
    println!("First: {}", first);
}
