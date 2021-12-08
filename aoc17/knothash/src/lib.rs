extern crate hex;

use itertools::Itertools;

pub fn twist(list: &mut Vec<u32>, pos: usize, len: usize) {
    let size = list.len();
    for i in 0..len / 2 {
        list.swap((pos + i) % size, (pos + len - i - 1) % size);
    }
}

pub fn knot_hash(input: &str) -> String {
    let mut lengths: Vec<usize> = input.trim().chars().map(|c| c as usize).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut list: Vec<u32> = (0..256).collect();

    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for len in &lengths {
            twist(&mut list, pos, *len);
            pos += (len + skip) % list.len();
            skip += 1;
        }
    }

    let dense_hash: Vec<u8> = list
        .iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk.fold(0, |acc, x| acc ^ x))
        .map(|x| x as u8)
        .collect();

    hex::encode(dense_hash)
}
