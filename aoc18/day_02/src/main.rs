use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let twices = input
        .trim()
        .lines()
        .filter(|&l| {
            l.chars()
                .any(|c1| l.chars().filter(|&c2| c1 == c2).count() == 2)
        })
        .count();
    let thrices = input
        .trim()
        .lines()
        .filter(|&l| {
            l.chars()
                .any(|c1| l.chars().filter(|&c2| c1 == c2).count() == 3)
        })
        .count();

    println!("First: {}", twices * thrices);

    let lines: Vec<_> = input.lines().collect();

    for i in 0..lines.len() - 2 {
        for j in i + 1..lines.len() - 1 {
            if dst(lines[i], lines[j]) == 1 {
                print!("Second: ");
                print_same(lines[i], lines[j]);
            }
        }
    }
}

fn dst(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count()
}

fn print_same(s1: &str, s2: &str) {
    s1.chars()
        .zip(s2.chars())
        .filter(|(a, b)| a == b)
        .for_each(|(a, _)| print!("{}", a));
    println!();
}
