use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let area: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let first = evolution(&area, 10);
    println!("First: {}", first);
    let second = evolution(&area, 1_000_000_000);
    println!("Second: {}", second);
}

fn evolution(area_init: &Vec<Vec<char>>, minutes: i64) -> usize {
    let mut area = area_init.to_vec();
    let mut results = HashMap::new();

    let mut k: i64 = 0;
    while k < minutes {
        let area_prev = area.clone();

        for x in 0..area_prev.len() {
            for y in 0..area[x].len() {
                if area_prev[x][y] == '.' && count(&area_prev, x, y, '|') >= 3 {
                    area[x][y] = '|';
                } else if area_prev[x][y] == '|' && count(&area_prev, x, y, '#') >= 3 {
                    area[x][y] = '#';
                } else if area_prev[x][y] == '#'
                    && (count(&area_prev, x, y, '#') == 0 || count(&area_prev, x, y, '|') == 0)
                {
                    area[x][y] = '.';
                }
            }
        }

        match results.get(&area) {
            Some(k_prev) => {
                let rem = (minutes - k) % (k - k_prev);
                k = minutes - rem;
            }
            None => {}
        }
        results.insert(area.clone(), k);
        k += 1;
    }
    let lumber: usize = area
        .iter()
        .map(|row| row.iter().filter(|&x| *x == '#').count())
        .sum();
    let wood: usize = area
        .iter()
        .map(|row| row.iter().filter(|&x| *x == '|').count())
        .sum();
    lumber * wood
}

#[allow(dead_code)]
fn print(area: &Vec<Vec<char>>) {
    println!();
    for row in area {
        for x in row {
            print!("{}", x);
        }
        println!();
    }
}

fn count(area: &Vec<Vec<char>>, x: usize, y: usize, content: char) -> usize {
    let mut res = 0;
    let x = x as i32;
    let y = y as i32;
    for &(i, j) in [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
    .iter()
    {
        if i >= 0
            && (i as usize) < area.len()
            && j >= 0
            && (j as usize) < area[0].len()
            && area[i as usize][j as usize] == content
        {
            res += 1;
        }
    }
    res
}
