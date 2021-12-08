use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Rule {
    field: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

fn valid(val: u32, rule: &Rule) -> bool {
    val >= rule.range1.0 && val <= rule.range1.1 || val >= rule.range2.0 && val <= rule.range2.1
}

fn parse_rule(line: &str) -> Rule {
    let parts: Vec<&str> = line.split(':').collect();
    let ranges: Vec<&str> = parts[1].split("or").map(|p| p.trim()).collect();
    let r1: Vec<u32> = ranges[0].split('-').map(|n| n.parse().unwrap()).collect();
    let r2: Vec<u32> = ranges[1].split('-').map(|n| n.parse().unwrap()).collect();

    Rule {
        field: parts[0].to_string(),
        range1: (r1[0], r1[1]),
        range2: (r2[0], r2[1]),
    }
}

fn singleton(set: &HashSet<usize>) -> Option<usize> {
    match set.len() {
        1 => set.iter().nth(0).cloned(),
        _ => None,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Rule> = parts[0].lines().map(|line| parse_rule(line)).collect();

    let my_ticket: Vec<u32> = parts[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut tickets: Vec<Vec<u32>> = parts[2]
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let rate: u32 = tickets
        .iter()
        .map(|t| {
            t.iter()
                .filter(|&n| !rules.iter().any(|r| valid(*n, r)))
                .sum::<u32>()
        })
        .sum();
    println!("First: {}", rate);

    tickets = tickets
        .iter()
        .filter(|t| t.iter().all(|&n| rules.iter().any(|r| valid(n, r))))
        .cloned()
        .collect();

    let mut rules_map: HashMap<String, HashSet<usize>> = rules
        .iter()
        .map(|r| {
            (
                r.field.clone(),
                (0..rules.len())
                    .filter(|&i| tickets.iter().all(|t| valid(t[i], &r)))
                    .collect(),
            )
        })
        .collect();

    while !rules_map.iter().all(|(_, indices)| indices.len() == 1) {
        let unique: HashSet<usize> = rules_map
            .iter()
            .filter_map(|(_, indices)| singleton(indices))
            .collect();

        for (_, indices) in rules_map.iter_mut().filter(|(_, i)| i.len() > 1) {
            indices.retain(|i| !unique.contains(i));
        }
    }

    let second = rules_map
        .iter()
        .filter(|(n, _)| n.starts_with("departure"))
        .fold(1, |prod, (_, i)| {
            prod * my_ticket[singleton(&i).unwrap()] as u64
        });
    println!("Second: {}", second);
}
