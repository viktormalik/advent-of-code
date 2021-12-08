extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Guard {
    id: u32,
    total: u32,
    mins: HashMap<u32, u32>,
    min_most: (u32, u32),
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut sorted: Vec<_> = input.trim().split("\n").collect();
    sorted.sort();
    let mut guards = HashMap::new();

    let re = Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)$").unwrap();
    let re_guard = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut active: u32 = 0;
    let mut last_min: u32 = 0;

    for line in &sorted {
        let cap = re.captures(line).unwrap();

        if re_guard.is_match(&cap[6]) {
            active = re_guard.captures(&cap[6]).unwrap()[1]
                .parse::<u32>()
                .unwrap();
            continue;
        } else if &cap[6] == "falls asleep" {
            last_min = cap[5].parse::<u32>().unwrap();
        } else if &cap[6] == "wakes up" {
            let guard = guards.entry(active).or_insert(Guard {
                id: active,
                total: 0,
                mins: HashMap::new(),
                min_most: (0, 0),
            });
            let min = cap[5].parse::<u32>().unwrap();
            guard.total += min - last_min;

            for m in last_min..min {
                *guard.mins.entry(m).or_insert(0) += 1;
            }
        }
    }

    for g in guards.values_mut() {
        g.min_most = deref_tuple(g.mins.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap());
    }

    let first = guards
        .values()
        .max_by(|a, b| a.total.cmp(&b.total))
        .unwrap();
    println!("First: {}", first.id * first.min_most.0);

    let second = guards
        .values()
        .max_by(|a, b| a.min_most.1.cmp(&b.min_most.1))
        .unwrap();
    println!("Second: {}", second.id * second.min_most.0);
}

fn deref_tuple(t: (&u32, &u32)) -> (u32, u32) {
    (*t.0, *t.1)
}
