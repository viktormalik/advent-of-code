use std::fs;

fn parse(line: &str) -> (u32, u32) {
    let parts: Vec<u32> = line
        .trim()
        .split(": ")
        .map(|n| n.parse().unwrap())
        .collect();
    (parts[0], parts[1])
}

fn caught(layers: &Vec<(u32, u32)>, delay: u32) -> Vec<&(u32, u32)> {
    layers
        .iter()
        .filter(|(depth, range)| (depth + delay) % ((range - 1) * 2) == 0)
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let layers: Vec<(u32, u32)> = input.lines().map(|l| parse(l)).collect();

    let first: u32 = caught(&layers, 0)
        .iter()
        .map(|(depth, range)| depth * range)
        .sum();
    println!("First: {}", first);

    for d in 0..10000000 {
        if caught(&layers, d).is_empty() {
            println!("Second: {}", d);
            break;
        }
    }
}
