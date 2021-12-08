use intcode::*;
use std::cmp::min;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut starts = vec![];
    let mut ends = vec![];
    let mut count = 0;
    for x in 0i32..10000 {
        let mut start: i32 = -1;
        let mut end: i32 = -1;
        for y in 0i32..10000 {
            if x > 0 && y < starts[x as usize - 1] - 1 {
                continue;
            }
            let mut prog = IntcodeProgram::load(&input);
            prog.run(&vec![y as i64, x as i64]);
            if prog.stdout.pop().unwrap() == 1 {
                if start == -1 {
                    start = y;
                    if x - 99 >= 0 && ends[(x - 99) as usize] >= y + 99 {
                        println!("Second: {}", y * 10000 + (x - 99));
                        return;
                    }
                }
                end = y;
                count += 1;
            } else if end != -1 {
                break;
            }
        }
        starts.push(start);
        ends.push(end);
        if x == 49 {
            let first: i32 = starts
                .iter()
                .zip(ends.iter())
                .map(|(&s, &e)| match s >= 0 && s < 50 {
                    true => min(e, 49) - min(s, 49) + 1,
                    false => 0,
                })
                .sum();
            println!("First: {}", first);
        }
    }
    println!("First: {}", count);
}
