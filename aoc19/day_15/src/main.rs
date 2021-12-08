use intcode::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn get_next(pos: &(i32, i32), dir: i32) -> (i32, i32) {
    match dir {
        1 => (pos.0, pos.1 - 1),
        2 => (pos.0, pos.1 + 1),
        3 => (pos.0 - 1, pos.1),
        4 => (pos.0 + 1, pos.1),
        _ => *pos,
    }
}

fn get_prev(pos: &(i32, i32), dir: i32) -> (i32, i32) {
    let reverse_dir = match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => dir,
    };
    get_next(pos, reverse_dir)
}

fn update_to_explore(
    to_explore: &mut Vec<(i32, i32)>,
    positions: &HashMap<(i32, i32), i32>,
    current: &(i32, i32),
) {
    for dir in 1..5 {
        let next = get_next(current, dir);
        if !positions.contains_key(&next) {
            to_explore.push(next);
        }
    }
}

fn restore_path(from: &(i32, i32), to: &(i32, i32), pred: &HashMap<(i32, i32), i32>) -> Vec<i64> {
    let mut res = vec![];
    let mut current = *to;
    while current != *from {
        let dir = pred.get(&current).unwrap();
        res.push(*dir as i64);
        current = get_prev(&current, *dir);
    }
    res.reverse();
    res
}

fn get_path(
    from: &(i32, i32),
    to: Option<(i32, i32)>,
    positions: &HashMap<(i32, i32), i32>,
) -> Vec<i64> {
    let mut dsts: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pred: HashMap<(i32, i32), i32> = HashMap::new();

    let mut queue: HashSet<(i32, i32)> = positions
        .iter()
        .filter(|(_, &v)| v > 0)
        .map(|(&pos, _)| pos)
        .collect();

    dsts.insert(*from, 0);

    while queue.len() > 0 {
        let current = *queue
            .iter()
            .min_by_key(|p| match dsts.get(p) {
                Some(&d) => d,
                None => 1000,
            })
            .unwrap();
        queue.remove(&current);

        if to.is_some() && current == to.unwrap() {
            return restore_path(from, &current, &pred);
        }

        let current_dst = *dsts.get(&current).unwrap();

        for dir in 1..5 {
            let next = get_next(&current, dir);
            if positions.contains_key(&next) && *positions.get(&next).unwrap() > 0 {
                let next_dst = current_dst + 1;
                if !dsts.contains_key(&next) || *dsts.get(&next).unwrap() > next_dst {
                    *dsts.entry(next).or_insert(next_dst) = next_dst;
                    *pred.entry(next).or_insert(dir) = dir;
                }
            }
        }
    }

    let max = dsts.iter().max_by_key(|&(_, dst)| dst).unwrap().0;
    restore_path(from, &max, &pred)
}

#[allow(dead_code)]
fn print(positions: &HashMap<(i32, i32), i32>) {
    let minx = positions.keys().map(|&(x, _)| x).min().unwrap();
    let maxx = positions.keys().map(|&(x, _)| x).max().unwrap();
    let miny = positions.keys().map(|&(_, y)| y).min().unwrap();
    let maxy = positions.keys().map(|&(_, y)| y).max().unwrap();

    for y in 0..maxy - miny + 1 {
        for x in 0..maxx - minx + 1 {
            if x + minx == 0 && y + miny == 0 {
                print!("{}", 'o');
            } else {
                print!(
                    "{}",
                    match positions.get(&(x + minx, y + miny)) {
                        Some(x) => match x {
                            0 => '#',
                            1 => '.',
                            2 => 'X',
                            _ => ' ',
                        },
                        None => ' ',
                    }
                );
            }
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut positions = HashMap::new();
    let mut current = (0, 0);
    let mut oxygen_system = (0, 0);
    positions.insert(current, 1);

    let mut to_explore: Vec<(i32, i32)> = vec![];
    update_to_explore(&mut to_explore, &positions, &current);

    let mut prog = IntcodeProgram::load(&input);

    while to_explore.len() > 0 {
        let next = to_explore.pop().unwrap();

        if positions.contains_key(&next) {
            continue;
        }

        positions.insert(next, 1);

        let path = get_path(&current, Some(next), &positions);
        for i in 0..path.len() - 1 {
            prog.run(&vec![path[i]]);
            current = get_next(&current, path[i] as i32);
        }

        prog.run(&vec![path[path.len() - 1]]);

        let status = prog.stdout.pop().unwrap() as i32;
        prog.stdout.clear();
        *positions.get_mut(&next).unwrap() = status;

        if status > 0 {
            current = next;
            update_to_explore(&mut to_explore, &positions, &current);
        }
        if status == 2 {
            oxygen_system = current;
        }
    }

    println!(
        "First: {}",
        get_path(&(0, 0), Some(oxygen_system), &positions).len()
    );
    println!(
        "Second: {}",
        get_path(&oxygen_system, None, &positions).len()
    );
}
