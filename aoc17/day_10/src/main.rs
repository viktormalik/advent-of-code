use knothash::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .map(|len| len.parse().unwrap())
        .collect();

    let mut list: Vec<u32> = (0..256).collect();

    let mut pos = 0;
    let mut skip = 0;
    for len in lengths {
        twist(&mut list, pos, len);
        pos += (len + skip) % list.len();
        skip += 1;
    }

    println!("First: {}", list[0] * list[1]);

    let hash = knot_hash(&input);
    println!("Second: {}", hash);
}
