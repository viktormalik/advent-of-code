use std::fs;

fn next_dest(dest: usize, max: usize) -> usize {
    match dest - 1 {
        0 => max,
        x => x,
    }
}

fn mix(cups: &Vec<usize>, iterations: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![0; cups.len() + 1];
    for i in 0..cups.len() {
        indices[cups[i]] = i;
    }

    let mut next: Vec<usize> = (0..cups.len()).map(|n| (n + 1) % cups.len()).collect();

    let mut curr = 0;

    for _ in 0..iterations {
        let a = next[curr];
        let b = next[a];
        let c = next[b];

        let mut dest_cup = next_dest(cups[curr], cups.len());
        while dest_cup == cups[a] || dest_cup == cups[b] || dest_cup == cups[c] {
            dest_cup = next_dest(dest_cup, cups.len());
        }
        let dest = indices[dest_cup];

        next[curr] = next[c];
        next[c] = next[dest];
        next[dest] = a;

        curr = next[curr];
    }

    // Returns 9 cups following cup no. 1
    let mut res = vec![];
    let mut cup = cups[next[indices[1]]];
    for _ in 0..8 {
        res.push(cup);
        cup = cups[next[indices[cup]]];
    }
    res
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let cups_small: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let first = mix(&cups_small, 100);
    print!("First: ");
    for c in first {
        print!("{}", c);
    }
    println!();

    let cups_big: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(10..=1_000_000)
        .collect();

    let second = mix(&cups_big, 10_000_000);
    println!("Second: {}", second[0] * second[1]);
}
