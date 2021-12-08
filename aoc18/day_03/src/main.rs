extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut bits = vec![vec![0; 1000]; 1000];
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let mut candidates = HashSet::new();

    for line in input.trim().lines() {
        if line.is_empty() {
            continue;
        }

        let cap = re.captures(line).unwrap();
        let claim = Claim {
            id: cap[1].parse::<usize>().unwrap(),
            x: cap[2].parse::<usize>().unwrap(),
            y: cap[3].parse::<usize>().unwrap(),
            w: cap[4].parse::<usize>().unwrap(),
            h: cap[5].parse::<usize>().unwrap(),
        };

        let mut all_one = true;
        for i in claim.x..claim.x + claim.w {
            for j in claim.y..claim.y + claim.h {
                bits[i][j] += 1;
                if !(bits[i][j] == 1) {
                    all_one = false;
                    candidates.retain(|c: &Claim| {
                        !(i >= c.x && i < (c.x + c.w) && j >= c.y && j < (c.y + c.h))
                    });
                }
            }
        }
        if all_one {
            candidates.insert(claim);
        }
    }

    let res: usize = bits
        .iter()
        .map(|r| r.iter().filter(|&v| v > &1).count())
        .sum();
    println!("First: {}", res);

    assert_eq!(candidates.len(), 1);
    for c in &candidates {
        println!("Second: {}", c.id);
    }
}
