use std::fs;

fn slope(r_inc: usize, c_inc: usize, trees: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let mut c = 0;
    for r in (0..trees.len()).step_by(r_inc) {
        if trees[r][c % trees[r].len()] == '#' {
            count += 1;
        }
        c += c_inc;
    }
    count
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let trees: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    println!("First: {}", slope(1, 3, &trees));

    let second = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|(r, c)| slope(*r as usize, *c as usize, &trees))
        .fold(1, |res, x| res * x);
    println!("Second: {}", second);
}
