use std::collections::HashMap;
use std::fs;

type MaskMapping = fn(char) -> char;

fn get_mask(mask: &str, map_fn: MaskMapping) -> u64 {
    u64::from_str_radix(&mask.chars().map(map_fn).collect::<String>(), 2).unwrap()
}

fn expand(val: u64, mask: &str) -> Vec<u64> {
    let val_bin = format!("{:036b}", val);
    let mut result: Vec<String> = vec!["".to_string()];

    for (i, bit) in mask.chars().enumerate() {
        match bit {
            '0' => {
                for res in &mut result {
                    res.push(val_bin.as_bytes()[i] as char);
                }
            }
            '1' => {
                for res in &mut result {
                    res.push('1');
                }
            }
            _ => {
                let mut new_result = vec![];
                for res in result {
                    new_result.push(res.clone() + "0");
                    new_result.push(res.clone() + "1");
                }
                result = new_result.clone();
            }
        }
    }

    result
        .iter()
        .map(|x| u64::from_str_radix(x, 2).unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut mem = HashMap::new();
    for line in input.lines() {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        if cmd[0] == "mask" {
            and_mask = get_mask(cmd[2], |c| match c {
                '0' => '0',
                _ => '1',
            });
            or_mask = get_mask(cmd[2], |c| match c {
                '1' => '1',
                _ => '0',
            });
        } else {
            let addr: u64 = cmd[0][4..cmd[0].len() - 1].parse().unwrap();
            let val: u64 = cmd[2].parse::<u64>().unwrap() & and_mask | or_mask;
            mem.insert(addr, val);
        }
    }
    println!("First: {}", mem.values().sum::<u64>());

    let mut mem = HashMap::new();
    let mut mask = "";
    for line in input.lines() {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        if cmd[0] == "mask" {
            mask = cmd[2];
        } else {
            let addr: u64 = cmd[0][4..cmd[0].len() - 1].parse().unwrap();
            let val: u64 = cmd[2].parse::<u64>().unwrap();
            for a in expand(addr, mask) {
                mem.insert(a, val);
            }
        }
    }
    println!("Second: {}", mem.values().sum::<u64>());
}
