use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut banks: Vec<u32> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut states = HashMap::new();

    let mut i = 0;
    while !states.contains_key(&banks) {
        states.insert(banks.clone(), i);

        let bank = banks
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .max_by(|b1, b2| b1.1.cmp(&b2.1).then(b2.0.cmp(&b1.0)))
            .unwrap();

        banks[bank.0] = 0;

        // Collect list of indices to be updated
        let indices: Vec<usize> = banks
            .iter()
            .enumerate()
            .cycle()
            .skip(bank.0 + 1)
            .take(bank.1 as usize)
            .map(|b| b.0)
            .collect();

        for i in indices {
            banks[i] += 1;
        }

        i += 1;
    }

    println!("First: {}", i);
    println!("Second: {}", i - states.get(&banks).unwrap());
}
