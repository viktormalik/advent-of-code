use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn match_string(
    input: &str,
    nonterm_rules: &HashMap<u32, Vec<Vec<u32>>>,
    term_rules: &HashMap<u32, char>,
) -> bool {
    let msg: Vec<char> = input.chars().collect();
    let mut states: Vec<(VecDeque<u32>, usize)> = vec![(VecDeque::new(), 0)];
    states[0].0.push_front(0);
    while states.len() > 0 {
        let mut state = states.pop().unwrap();
        let nonterm = state.0.pop_front().unwrap();
        if term_rules.contains_key(&nonterm) {
            if *term_rules.get(&nonterm).unwrap() == msg[state.1] {
                state.1 += 1;
                if state.1 == msg.len() {
                    if state.0.len() == 0 {
                        return true;
                    }
                } else if state.0.len() > 0 {
                    states.push(state);
                }
            }
        } else {
            for rhs in nonterm_rules.get(&nonterm).unwrap() {
                let mut new_state = state.clone();
                for n in rhs.iter().rev() {
                    new_state.0.push_front(*n);
                }
                states.push(new_state);
            }
        }
    }
    false
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let rules_input = input.split("\n\n").nth(0).unwrap();

    let mut nonterm_rules: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut term_rules: HashMap<u32, char> = HashMap::new();
    for line in rules_input.lines() {
        let keyval: Vec<&str> = line.split(':').collect();
        let rule: u32 = keyval[0].parse().unwrap();
        let parts: Vec<&str> = keyval[1].split('|').collect();

        for p in parts {
            if p.trim().starts_with("\"") {
                term_rules.insert(rule, p.trim().as_bytes()[1] as char);
            } else {
                nonterm_rules.entry(rule).or_insert(vec![]).push(
                    p.trim()
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                );
            }
        }
    }

    let first = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .trim()
        .lines()
        .filter(|l| match_string(l, &nonterm_rules, &term_rules))
        .count();
    println!("First: {}", first);

    *nonterm_rules.get_mut(&8).unwrap() = vec![vec![42], vec![42, 8]];
    *nonterm_rules.get_mut(&11).unwrap() = vec![vec![42, 31], vec![42, 11, 31]];
    let second = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .trim()
        .lines()
        .filter(|l| match_string(l, &nonterm_rules, &term_rules))
        .count();
    println!("Second: {}", second);
}
