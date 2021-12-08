use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn neighbors(pos: &Vec<i32>, n: usize) -> Vec<Vec<i32>> {
    (0..n)
        .map(|_| (-1..=1).into_iter())
        .multi_cartesian_product()
        .filter(|diff| diff.iter().any(|&x| x != 0))
        .map(|diff| pos.iter().zip(diff).map(|(p, d)| p + d).collect())
        .collect()
}

fn apply(cubes: &mut HashSet<Vec<i32>>, prev_cubes: &HashSet<Vec<i32>>, pos: &Vec<i32>) {
    let cnt = neighbors(pos, pos.len())
        .iter()
        .filter(|pos| prev_cubes.contains(*pos))
        .count();
    if prev_cubes.contains(pos) && cnt != 2 && cnt != 3 {
        cubes.remove(pos);
    } else if !prev_cubes.contains(pos) && cnt == 3 {
        cubes.insert(pos.clone());
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut cubes_3d: HashSet<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(j, _)| vec![i as i32, j as i32, 0])
        })
        .flatten()
        .collect();
    let mut cubes_4d: HashSet<Vec<i32>> = cubes_3d
        .iter()
        .map(|c| c.iter().chain([0].iter()).cloned().collect())
        .collect();

    let mut min = vec![0, 0, 0, 0];
    let mut max = vec![
        input.lines().count() as i32 - 1,
        input.lines().nth(0).unwrap().chars().count() as i32 - 1,
        0,
        0,
    ];

    for _ in 0..6 {
        for p in &mut min {
            *p -= 1;
        }
        for p in &mut max {
            *p += 1;
        }
        let prev_cubes_3d = cubes_3d.clone();
        let prev_cubes_4d = cubes_4d.clone();
        for x in min[0]..=max[0] {
            for y in min[1]..=max[1] {
                for z in min[2]..=max[2] {
                    apply(&mut cubes_3d, &prev_cubes_3d, &vec![x, y, z]);
                    for w in min[3]..=max[3] {
                        apply(&mut cubes_4d, &prev_cubes_4d, &vec![x, y, z, w]);
                    }
                }
            }
        }
    }

    println!("First: {}", cubes_3d.len());
    println!("Second: {}", cubes_4d.len());
}
