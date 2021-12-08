use std::collections::HashMap;
use std::fs;

fn parse_rule(line: &str) -> (String, char) {
    let mut s = line.split(" => ");
    (
        s.next().unwrap().to_string(),
        s.next().unwrap().chars().nth(0).unwrap(),
    )
}

fn step(index: usize, state: &Vec<char>, rules: &HashMap<String, char>) -> char {
    if index < 2 || index >= state.len() - 2 {
        return state[index];
    }

    let pattern: String = state[index - 2..=index + 2].iter().collect();
    match rules.get(&pattern) {
        Some(&c) => c,
        None => '.',
    }
}

fn equal(current: &Vec<char>, prev: &Vec<char>) -> Option<i64> {
    match current
        .iter()
        .skip_while(|&c| *c == '.')
        .zip(prev.iter().skip_while(|&c| *c == '.'))
        .all(|(c1, c2)| c1 == c2)
    {
        true => Some(
            current.iter().take_while(|&c| *c == '.').count() as i64
                - prev.iter().take_while(|&c| *c == '.').count() as i64,
        ),
        false => None,
    }
}

fn evolution(
    init_state: &str,
    rules: &HashMap<String, char>,
    generations: u64,
) -> (Vec<char>, i64) {
    let mut state: Vec<char> = init_state.chars().collect();
    let mut start = 0;

    for g in 0..generations {
        let prev_state = state.clone();
        state = ['.', '.', '.']
            .iter()
            .chain(state.iter())
            .chain(['.', '.', '.'].iter())
            .cloned()
            .collect();
        state = (0..state.len()).map(|i| step(i, &state, rules)).collect();
        start -= 3;

        let shift = equal(&state, &prev_state);
        if shift.is_some() {
            start += (generations - g - 1) as i64 * (shift.unwrap() - 3);
            return (state, start);
        }
    }

    (state, start)
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let init_state: String = input.lines().nth(0).unwrap().chars().skip(15).collect();
    let rules: HashMap<String, char> = input
        .trim()
        .lines()
        .skip(2)
        .map(|line| parse_rule(line))
        .collect();

    let first_state = evolution(&init_state, &rules, 20);
    let first: i64 = first_state
        .0
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '#')
        .map(|(i, _)| i as i64 + first_state.1)
        .sum();
    println!("First: {}", first);

    let second_state = evolution(&init_state, &rules, 50000000000);
    let second: i64 = second_state
        .0
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '#')
        .map(|(i, _)| i as i64 + second_state.1)
        .sum();
    println!("Second: {}", second);
}
