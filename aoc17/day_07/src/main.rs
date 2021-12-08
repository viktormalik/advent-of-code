use std::collections::HashMap;
use std::fs;

struct Program {
    name: String,
    weight: u32,
    stack_weight: u32,
    children: Vec<String>,
}

fn parse(line: &str) -> Program {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Program {
        name: parts[0].to_string(),
        weight: parts[1][1..parts[1].len() - 1].parse().unwrap(),
        stack_weight: 0,
        children: match parts.len() > 2 {
            true => parts[3..]
                .iter()
                .map(|p| {
                    match p.ends_with(",") {
                        true => &p[..p.len() - 1],
                        false => *p,
                    }
                    .to_string()
                })
                .collect(),
            false => vec![],
        },
    }
}

fn stack_weight(name: String, progs: &HashMap<String, Program>) -> u32 {
    let prog = progs.get(&name).unwrap();
    match prog.children.len() {
        0 => prog.weight,
        _ => {
            prog.weight
                + prog
                    .children
                    .iter()
                    .map(|c| stack_weight(c.to_string(), progs))
                    .sum::<u32>()
        }
    }
}

fn diff_child<'a>(prog: &Program, progs: &'a HashMap<String, Program>) -> Option<&'a Program> {
    match prog.children.iter().find(|p| {
        prog.children
            .iter()
            .filter(|c| progs.get(*c).unwrap().stack_weight == progs.get(*p).unwrap().stack_weight)
            .count()
            == 1
    }) {
        Some(name) => progs.get(name),
        None => None,
    }
}

fn find_unweighed<'a>(prog: &'a Program, progs: &'a HashMap<String, Program>) -> &'a Program {
    match diff_child(prog, progs) {
        Some(p) => find_unweighed(p, progs),
        None => prog,
    }
}

fn weight_diff(prog: &Program, progs: &HashMap<String, Program>) -> i32 {
    let unweighed = diff_child(prog, progs).unwrap();
    progs
        .get(
            prog.children
                .iter()
                .find(|&c| *c != unweighed.name)
                .unwrap(),
        )
        .unwrap()
        .stack_weight as i32
        - unweighed.stack_weight as i32
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut progs = HashMap::new();

    for line in input.lines() {
        let p = parse(line);
        progs.insert(p.name.clone(), p);
    }

    let prog_names: Vec<String> = progs.keys().map(|k| k.clone()).collect();
    for p in prog_names {
        progs.get_mut(&p).unwrap().stack_weight = stack_weight(p.clone(), &progs);
    }

    let root = progs.values().max_by_key(|p| p.stack_weight).unwrap();
    println!("First: {}", root.name);

    let unweighed = find_unweighed(root, &progs);
    let diff = weight_diff(root, &progs);
    println!("Second: {}", unweighed.weight as i32 + diff);
}
