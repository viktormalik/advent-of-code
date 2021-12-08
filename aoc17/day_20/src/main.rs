extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Part {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
    acc: (i32, i32, i32),
}

fn manhattan(coor: &(i32, i32, i32)) -> i32 {
    coor.0.abs() + coor.1.abs() + coor.2.abs()
}

fn parse(line: &str, re: &Regex) -> Part {
    let parts: Vec<(i32, i32, i32)> = re
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|cap| cap.unwrap().as_str().parse().unwrap())
        .tuples::<(_, _, _)>()
        .collect();
    Part {
        pos: parts[0],
        vel: parts[1],
        acc: parts[2],
    }
}

fn to_uint(x: Option<f32>) -> Option<u32> {
    match x {
        Some(v) => match v >= 0.0 && v.fract() == 0.0 {
            true => Some(v as u32),
            false => None,
        },
        None => None,
    }
}

fn solve(coor1: &(i32, i32, i32), coor2: &(i32, i32, i32)) -> (Option<u32>, Option<u32>) {
    let a: f32 = (coor1.0 - coor2.0) as f32 / 2.0;
    let b: f32 = (coor1.1 - coor2.1) as f32 + a;
    let c: f32 = (coor1.2 - coor2.2) as f32;

    let mut res1 = None;
    let mut res2 = None;

    if a == 0.0 {
        if b == 0.0 {
            if c == 0.0 {
                res1 = Some(0.0);
                res2 = Some(0.0);
            }
        } else {
            res1 = Some(-c / b);
        }
    } else {
        let d = b * b - 4.0 * a * c;
        if d == 0.0 {
            res1 = Some(-b / (2.0 * a));
        } else if d > 0.0 {
            res1 = Some((-b + d.sqrt()) / (2.0 * a));
            res2 = Some((-b - d.sqrt()) / (2.0 * a));
        }
    }
    (to_uint(res1), to_uint(res2))
}

fn find_collision(p1: &Part, p2: &Part) -> Option<u32> {
    let cols_x = solve(
        &(p1.acc.0, p1.vel.0, p1.pos.0),
        &(p2.acc.0, p2.vel.0, p2.pos.0),
    );
    let cols_y = solve(
        &(p1.acc.1, p1.vel.1, p1.pos.1),
        &(p2.acc.1, p2.vel.1, p2.pos.1),
    );
    let cols_z = solve(
        &(p1.acc.2, p1.vel.2, p1.pos.2),
        &(p2.acc.2, p2.vel.2, p2.pos.2),
    );

    for x_col in &[cols_x.0, cols_x.1] {
        for y_col in &[cols_y.0, cols_y.1] {
            for z_col in &[cols_z.0, cols_z.1] {
                if x_col.is_none() || y_col.is_none() || z_col.is_none() {
                    continue;
                }
                let x = x_col.unwrap();
                let y = y_col.unwrap();
                let z = z_col.unwrap();

                if (y == 0 && z == 0) || (y == 0 && x == z) || (z == 0 && x == y) {
                    return Some(x);
                } else if (x == 0 && z == 0) || (x == 0 && y == z) || (z == 0 && x == y) {
                    return Some(y);
                } else if (y == 0 && z == 0) || (x == 0 && y == z) || (y == 0 && x == z) {
                    return Some(z);
                }

                if x == y && x == z {
                    return Some(x);
                }
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let re = Regex::new(r"p=<(.*),(.*),(.*)>, v=<(.*),(.*),(.*)>, a=<(.*),(.*),(.*)>").unwrap();
    let mut particles: Vec<Part> = input.lines().map(|l| parse(l, &re)).collect();

    let first = particles
        .iter()
        .enumerate()
        .min_by(|(_, p1), (_, p2)| {
            manhattan(&p1.acc)
                .cmp(&manhattan(&p2.acc))
                .then(manhattan(&p1.vel).cmp(&manhattan(&p2.vel)))
                .then(manhattan(&p1.pos).cmp(&manhattan(&p2.pos)))
        })
        .unwrap()
        .0;
    println!("First: {}", first);

    let collisions: Vec<(Part, Part)> = particles
        .iter()
        .cartesian_product(particles.iter())
        .filter(|(p1, p2)| p1 != p2 && find_collision(p1, p2).is_some())
        .map(|(p1, p2)| (*p1, *p2))
        .collect();

    for c in collisions {
        if particles.contains(&c.0) {
            particles.remove(particles.iter().position(|&p| p == c.0).unwrap());
        }
        if particles.contains(&c.1) {
            particles.remove(particles.iter().position(|&p| p == c.1).unwrap());
        }
    }
    println!("Second: {}", particles.len());
}
