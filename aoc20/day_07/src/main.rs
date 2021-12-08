use std::collections::HashMap;
use std::collections::HashSet;

use std::fs;

fn parse_bag(bag: &str) -> (String, u32) {
    let parts: Vec<&str> = bag.trim().split_whitespace().collect();
    (
        parts[1].to_string() + " " + parts[2],
        parts[0].parse().unwrap(),
    )
}

fn parse_rule(line: &str) -> (String, Vec<(String, u32)>) {
    let parts: Vec<&str> = line.split("bags contain").collect();
    (
        parts[0].trim().to_string(),
        match parts[1].trim() {
            "no other bags." => vec![],
            _ => parts[1].split(',').map(|b| parse_bag(b)).collect(),
        },
    )
}

fn wrapper_bags<'a>(
    bag: &'a str,
    contained_in: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    match contained_in.get(bag) {
        Some(bags) => bags.iter().fold(bags.clone(), |res, b| {
            res.union(&wrapper_bags(b, contained_in)).cloned().collect()
        }),
        None => HashSet::new(),
    }
}

fn total_bags(bag: &str, bags: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    bags.get(bag).unwrap().iter().fold(0, |sum, (inner, cnt)| {
        sum + cnt + cnt * total_bags(inner, bags)
    })
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let bags: HashMap<String, Vec<(String, u32)>> = input.lines().map(|l| parse_rule(l)).collect();

    let mut contained_in = HashMap::new();
    for (name, inner) in &bags {
        for (bag, _) in inner {
            contained_in
                .entry(&bag[..])
                .or_insert(HashSet::new())
                .insert(&name[..]);
        }
    }

    let first = wrapper_bags("shiny gold", &contained_in).len();
    println!("First: {}", first);

    let second = total_bags("shiny gold", &bags);
    println!("Second: {}", second);
}
