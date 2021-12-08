use std::fs;

fn next(prev: u64, factor: u64) -> u64 {
    (prev * factor) % 2147483647
}

fn get_num(line: &str) -> u64 {
    line.split_whitespace().nth(4).unwrap().parse().unwrap()
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input");
    let a_start = get_num(input.lines().nth(0).unwrap());
    let b_start = get_num(input.lines().nth(1).unwrap());

    let first = (0..40000000)
        .scan((a_start, b_start), |(a, b), _| {
            *a = next(*a, 16807);
            *b = next(*b, 48271);
            Some((*a, *b))
        })
        .filter(|(a, b)| (a & 0xffff) == (b & 0xffff))
        .count();
    println!("First: {}", first);

    let second = (0..5000000)
        .scan((a_start, b_start), |(a, b), _| {
            while (*a = next(*a, 16807)) == () && *a % 4 != 0 {}
            while (*b = next(*b, 48271)) == () && *b % 8 != 0 {}
            Some((*a, *b))
        })
        .filter(|(a, b)| (a & 0xffff) == (b & 0xffff))
        .count();
    println!("Second: {}", second);
}
