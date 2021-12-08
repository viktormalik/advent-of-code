use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Reaction {
    amount: i64,
    srcs: HashSet<(String, i64)>,
}

fn parse_chemical(chemical: &str) -> (String, i64) {
    let amount = chemical.trim().split(" ").nth(0).unwrap().parse().unwrap();
    let kind = chemical.trim().split(" ").nth(1).unwrap().to_string();
    (kind, amount)
}

fn calc_ore(target_fuel: i64, reactions: &HashMap<String, Reaction>) -> i64 {
    let mut produced = HashMap::new();
    produced.insert("FUEL", target_fuel);

    loop {
        let next = produced
            .iter()
            .find(|(&chem, &amount)| chem != "ORE" && amount > 0);
        if next.is_none() {
            return *produced.get("ORE").unwrap();
        }
        let next_chem = next.unwrap().0.clone();
        let next_amount = next.unwrap().1;

        let reaction = reactions.get(&next_chem.to_string()).unwrap();
        let repeat = next_amount / reaction.amount
            + match next_amount % reaction.amount {
                0 => 0,
                _ => 1,
            };

        *produced.get_mut(next_chem).unwrap() -= reaction.amount * repeat;
        for src in reaction.srcs.iter() {
            *produced.entry(src.0.as_str()).or_insert(0) += src.1 * repeat;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut reactions = HashMap::new();

    for line in input.lines() {
        let dest = parse_chemical(line.split("=>").nth(1).unwrap());
        let reaction = reactions.entry(dest.0).or_insert(Reaction {
            amount: dest.1,
            srcs: HashSet::new(),
        });
        for src in line.split("=>").nth(0).unwrap().split(",") {
            reaction.srcs.insert(parse_chemical(src));
        }
    }

    let ore_per_fuel = calc_ore(1, &reactions);
    println!("First: {}", ore_per_fuel);

    let mut min = 1000000000000 / ore_per_fuel;
    let mut max = min * 2;

    while max - min != 1 {
        let mid = (min + max) / 2;
        let mid_fuel = calc_ore(mid, &reactions);
        if mid_fuel > 1000000000000 {
            max = mid;
        } else {
            min = mid;
        }
    }
    println!("Second: {}", min);
}
