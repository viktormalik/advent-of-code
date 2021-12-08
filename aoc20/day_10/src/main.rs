use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut adap: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    adap.push(0);
    adap.sort();
    adap.push(adap[adap.len() - 1] + 3);

    let ones = adap
        .iter()
        .tuple_windows()
        .filter(|&(x, y)| y - x == 1)
        .count();
    let threes = adap
        .iter()
        .tuple_windows()
        .filter(|&(x, y)| y - x == 3)
        .count();
    println!("First: {}", ones * threes);

    let mut res = 1u64;
    let mut i = 0;
    loop {
        if i >= adap.len() - 4 {
            break;
        }
        let val = adap[i];
        match adap[i + 1..=i + 3] {
            [x, y, z] if x == val + 1 && y == val + 2 && z == val + 3 => {
                res *= match adap[i + 4] {
                    x if x == val + 4 => 7,
                    x if x == val + 5 => 6,
                    _ => 4,
                };
                i += 3;
            }
            [x, y, _] if x == val + 1 && y == val + 2 || x == val + 1 && y == val + 3 => {
                res *= match adap[i + 4] {
                    x if x == val + 4 => 3,
                    _ => 2,
                };
                i += 2;
            }
            [x, y, _] if x == val + 2 && y == val + 3 => {
                res *= match adap[i + 4] {
                    x if x == val + 4 || x == val + 5 => 3,
                    _ => 2,
                };
                i += 2;
            }
            _ => i += 1,
        }
    }
    println!("Second: {}", res);
}
