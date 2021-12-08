extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Kind {
    IMMUNITY,
    INFECTION,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Group {
    kind: Kind,
    units: i32,
    hp: i32,
    weak: Vec<String>,
    immune: Vec<String>,
    damage: i32,
    attack: String,
    initiative: i32,
    power: i32,
    target: Option<usize>,
}

fn parse_group(line: &str, kind: Kind) -> Group {
    let regex = Regex::new(r"^(\d+) units each with (\d+) hit points (\(.+\) )?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)$").unwrap();
    let cap = regex.captures(line).unwrap();
    let units = cap[1].parse().unwrap();
    let damage = cap[4].parse().unwrap();
    let mut g = Group {
        kind: kind,
        units: units,
        hp: cap[2].parse().unwrap(),
        weak: Vec::new(),
        immune: Vec::new(),
        damage: damage,
        attack: cap[5].to_string(),
        initiative: cap[6].parse().unwrap(),
        power: units * damage,
        target: None,
    };
    if cap.get(3).is_some() {
        for special in cap[3]
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(';')
            .map(|s| s.trim())
        {
            if special.starts_with("weak to") {
                g.weak = special[8..]
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect();
            } else if special.starts_with("immune to") {
                g.immune = special[10..]
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }
    }
    g
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let split: Vec<&str> = input.split("\n\n").collect();
    let immunity: Vec<&str> = split[0].trim().lines().skip(1).collect();
    let infection: Vec<&str> = split[1].trim().lines().skip(1).collect();

    let mut groups_init: Vec<Group> = Vec::new();

    for i in immunity {
        groups_init.push(parse_group(i, Kind::IMMUNITY));
    }

    for i in infection {
        groups_init.push(parse_group(i, Kind::INFECTION));
    }

    let mut boost = 0;
    let mut smallest_winning = 0;
    let mut biggest_loosing = 0;
    let mut result = 0;

    loop {
        let mut groups = groups_init.to_vec();
        for i in groups.iter_mut().filter(|g| g.kind == Kind::IMMUNITY) {
            i.damage += boost;
            i.power = i.units * i.damage;
        }

        while groups.iter().any(|g| g.kind == Kind::IMMUNITY)
            && groups.iter().any(|g| g.kind == Kind::INFECTION)
        {
            let mut selection_order: Vec<usize> = { 0..groups.len() }.collect();
            selection_order.sort_by(|i, j| {
                (groups[*i].power, groups[*i].initiative)
                    .cmp(&(groups[*j].power, groups[*j].initiative))
                    .reverse()
            });

            let mut targets: HashSet<usize> = HashSet::new();
            for i in selection_order {
                let mut damages = vec![0; groups.len()];
                for j in 0..groups.len() {
                    if groups[i].kind != groups[j].kind {
                        damages[j] = match groups[j].immune.contains(&groups[i].attack) {
                            true => 0,
                            false => {
                                groups[i].power
                                    * match groups[j].weak.contains(&groups[i].attack) {
                                        true => 2,
                                        false => 1,
                                    }
                            }
                        };
                    }
                }
                let target = match groups
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| damages[*i] > 0 && !targets.contains(&i))
                    .max_by(|(i1, g1), (i2, g2)| {
                        (damages[*i1], g1.power, g1.initiative).cmp(&(
                            damages[*i2],
                            g2.power,
                            g2.initiative,
                        ))
                    }) {
                    Some((i, _)) => Some(i),
                    None => None,
                };
                groups[i].target = target;
                if target.is_some() {
                    targets.insert(target.unwrap());
                }
            }

            if groups.iter().all(|g| g.target.is_none()) {
                break;
            }

            let mut attack_order: Vec<usize> = { 0..groups.len() }.collect();
            attack_order
                .sort_by(|i, j| groups[*i].initiative.cmp(&groups[*j].initiative).reverse());

            let mut round_killed = 0;
            for i in attack_order {
                if groups[i].units > 0 && groups[i].target.is_some() {
                    let target = groups[i].target.unwrap();
                    let damage = groups[i].power
                        * match groups[target].weak.contains(&groups[i].attack) {
                            true => 2,
                            false => 1,
                        };
                    let units_killed = damage / groups[target].hp;
                    groups[target].units -= units_killed;
                    round_killed += units_killed;
                    groups[target].power = groups[target].units * groups[target].damage;
                }
            }
            if round_killed == 0 {
                break;
            }

            groups.retain(|g| g.units > 0);
        }

        if boost == 0 {
            boost = groups.iter().map(|g| g.units).sum::<i32>();
            println!("First: {}", boost);
        } else {
            if groups.iter().all(|g| g.kind == Kind::IMMUNITY) {
                result = groups.iter().map(|g| g.units).sum::<i32>();
                smallest_winning = boost;
                boost = (boost + biggest_loosing) / 2;
            } else {
                biggest_loosing = boost;
                boost = (boost + smallest_winning) / 2;
            }
            if smallest_winning - biggest_loosing == 1 {
                println!("Second: {}", result);
                return;
            }
        }
    }
}
