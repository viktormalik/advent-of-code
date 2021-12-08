use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn parse(line: &str) -> (u32, Vec<u32>) {
    let parts: Vec<&str> = line.split("<->").collect();
    (
        parts[0].trim().parse().unwrap(),
        parts[1]
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect(),
    )
}

fn get_group(prog: u32, graph: &HashMap<u32, Vec<u32>>, group: &mut HashSet<u32>) {
    if group.contains(&prog) {
        return;
    }

    group.insert(prog);
    for n in graph.get(&prog).unwrap() {
        get_group(*n, graph, group);
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let graph: HashMap<u32, Vec<u32>> = input.lines().map(|l| parse(&l)).collect();

    let mut progs: HashSet<u32> = graph.keys().map(|&k| k).collect();
    let mut groups = vec![];
    while !progs.is_empty() {
        let mut group = HashSet::new();
        get_group(*progs.iter().nth(0).unwrap(), &graph, &mut group);
        progs = progs.difference(&group).map(|&p| p).collect();
        groups.push(group);
    }

    println!(
        "First: {}",
        groups.iter().find(|g| g.contains(&0)).unwrap().len()
    );
    println!("Second: {}", groups.len());
}
