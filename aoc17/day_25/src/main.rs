use std::collections::HashMap;
use std::fs;

enum Dir {
    Left,
    Right,
}

struct Rule {
    val: u32,
    dir: Dir,
    next_state: char,
}

struct State {
    zero_rule: Rule,
    one_rule: Rule,
}

fn parse_rule(lines: &Vec<&str>) -> Rule {
    Rule {
        val: char::to_digit(lines[0].trim().chars().nth(18).unwrap(), 10).unwrap(),
        dir: match lines[1].split_whitespace().nth(6).unwrap() {
            "right." => Dir::Right,
            _ => Dir::Left,
        },
        next_state: lines[2].trim().chars().nth(22).unwrap(),
    }
}

fn apply_rule(rule: &Rule, slot: &mut u32, pos: &mut i32, next: &mut char) {
    *slot = rule.val;
    match rule.dir {
        Dir::Left => *pos -= 1,
        Dir::Right => *pos += 1,
    };
    *next = rule.next_state;
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut current: char = input.lines().nth(0).unwrap().chars().nth(15).unwrap();

    let steps: u32 = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();

    let mut states: HashMap<char, State> = HashMap::new();

    for s in input.split("\n\n").skip(1) {
        let lines: Vec<&str> = s.trim().lines().collect();

        let name = lines[0].chars().nth(9).unwrap();
        let zero_rule_lines = lines.iter().skip(2).take(3).cloned().collect();
        let one_rule_lines = lines.iter().skip(6).take(3).cloned().collect();

        states.insert(
            name,
            State {
                zero_rule: parse_rule(&zero_rule_lines),
                one_rule: parse_rule(&one_rule_lines),
            },
        );
    }

    let mut tape: HashMap<i32, u32> = HashMap::new();
    let mut pos = 0;

    for _ in 0..steps {
        let state = states.get(&current).unwrap();

        let slot = tape.entry(pos).or_insert(0);
        if *slot == 0 {
            apply_rule(&state.zero_rule, slot, &mut pos, &mut current);
        } else {
            apply_rule(&state.one_rule, slot, &mut pos, &mut current);
        }
    }

    let first = tape.iter().filter(|(_, &s)| s == 1).count();
    println!("First: {}", first);
}
