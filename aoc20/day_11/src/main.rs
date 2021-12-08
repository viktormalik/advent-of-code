use std::fs;

type CheckFn = fn((i32, i32), (i32, i32), &Vec<Vec<char>>) -> char;

fn check_adj(s: (i32, i32), diff: (i32, i32), seats: &Vec<Vec<char>>) -> char {
    let x = s.0 + diff.0;
    let y = s.1 + diff.1;
    if x >= 0 && y >= 0 && x < seats.len() as i32 && y < seats[0].len() as i32 {
        return seats[x as usize][y as usize];
    }
    return '.';
}

fn check_dir(s: (i32, i32), diff: (i32, i32), seats: &Vec<Vec<char>>) -> char {
    let mut x = s.0 + diff.0;
    let mut y = s.1 + diff.1;
    while x >= 0 && y >= 0 && x < seats.len() as i32 && y < seats[0].len() as i32 {
        match seats[x as usize][y as usize] {
            'L' | '#' => return seats[x as usize][y as usize],
            _ => {}
        }
        x += diff.0;
        y += diff.1;
    }
    return '.';
}

fn check_around(s: (i32, i32), seats: &Vec<Vec<char>>, check: CheckFn) -> usize {
    *&[0, 0, 1, 1, 1, -1, -1, -1]
        .iter()
        .zip(&[1, -1, 1, 0, -1, 1, 0, -1])
        .filter(|&(&x, &y)| check(s, (x, y), seats) == '#')
        .count()
}

fn simulate(start: &Vec<Vec<char>>, check: CheckFn, thresh: usize) -> usize {
    let mut seats = start.clone();
    let mut prev: Vec<Vec<char>> = vec![];

    while prev != seats {
        prev = seats.clone();
        for i in 0..seats.len() {
            for j in 0..seats[i].len() {
                let cnt = check_around((i as i32, j as i32), &prev, check);
                if seats[i][j] == 'L' && cnt == 0 {
                    seats[i][j] = '#';
                } else if seats[i][j] == '#' && cnt >= thresh {
                    seats[i][j] = 'L';
                }
            }
        }
    }

    seats.iter().flatten().filter(|&s| *s == '#').count()
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let seats: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let first = simulate(&seats, check_adj, 4);
    println!("First: {}", first);

    let second = simulate(&seats, check_dir, 5);
    println!("Second: {}", second);
}
