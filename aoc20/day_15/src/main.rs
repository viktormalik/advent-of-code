use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let nums_vec: Vec<u32> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut nums: HashMap<u32, usize> = nums_vec[0..nums_vec.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect();

    let mut last = nums_vec[nums_vec.len() - 1];
    for i in nums_vec.len() - 1..30000000 - 1 {
        if i == 2019 {
            println!("First: {}", last);
        }
        let new = match nums.get(&last) {
            Some(x) => (i - *x) as u32,
            None => 0,
        };
        nums.insert(last, i);
        last = new;
    }
    println!("Second: {}", last);
}
