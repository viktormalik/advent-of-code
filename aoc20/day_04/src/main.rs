use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn has_fields(pass: &HashMap<String, String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|f| pass.contains_key(*f))
}

fn valid_field(field: &(&String, &String)) -> bool {
    match &field.0[..] {
        "byr" => {
            let byr: u32 = field.1.parse().unwrap();
            byr >= 1920 && byr <= 2002
        }
        "iyr" => {
            let eyr: u32 = field.1.parse().unwrap();
            eyr >= 2010 && eyr <= 2020
        }
        "eyr" => {
            let eyr: u32 = field.1.parse().unwrap();
            eyr >= 2020 && eyr <= 2030
        }
        "hgt" => {
            let hgt: u32 = field
                .1
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            match &field.1[field.1.len() - 2..field.1.len()] {
                "cm" => hgt >= 150 && hgt <= 193,
                "in" => hgt >= 59 && hgt <= 79,
                _ => false,
            }
        }
        "hcl" => field.1.starts_with("#") && field.1.chars().skip(1).all(|c| c.is_digit(16)),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|clr| clr == field.1),
        "pid" => field.1.len() == 9 && field.1.chars().all(|c| c.is_digit(10)),
        "cid" => true,
        _ => false,
    }
}

fn valid(pass: &HashMap<String, String>) -> bool {
    has_fields(pass) && pass.iter().all(|field| valid_field(&field))
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let passports: Vec<HashMap<String, String>> = input
        .split("\n\n")
        .map(|pass| {
            pass.split_whitespace()
                .map(|part| {
                    part.split(':')
                        .map(|kv| kv.to_string())
                        .next_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let first = passports.iter().filter(|pass| has_fields(&pass)).count();
    println!("First: {}", first);

    let second = passports.iter().filter(|pass| valid(&pass)).count();
    println!("Second: {}", second);
}
