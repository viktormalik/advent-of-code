use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let answers: Vec<Vec<HashSet<char>>> = input
        .split("\n\n")
        .map(|grp| {
            grp.split_whitespace()
                .map(|a| a.chars().collect())
                .collect()
        })
        .collect();

    let first: usize = answers
        .iter()
        .map(|grp| {
            grp.iter()
                .fold(HashSet::new(), |res, a| res.union(a).cloned().collect())
        })
        .map(|answs| answs.len())
        .sum();
    println!("First: {}", first);

    let second: usize = answers
        .iter()
        .map(|grp| {
            grp.iter()
                .fold(('a'..='z').collect::<HashSet<_>>(), |res, a| {
                    res.intersection(a).cloned().collect()
                })
        })
        .map(|answs| answs.len())
        .sum();
    println!("Second: {}", second);
}
