extern crate num;
extern crate regex;

use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Moon {
    pos: Vec<i32>,
    vel: Vec<i32>,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let mut moons = vec![];
    let re = Regex::new(r"^<x=(.*), y=(.*), z=(.*)>$").unwrap();

    for line in input.lines() {
        moons.push(Moon {
            pos: re
                .captures_iter(line)
                .nth(0)
                .unwrap()
                .iter()
                .skip(1)
                .map(|m| m.unwrap().as_str().parse().unwrap())
                .collect(),
            vel: vec![0, 0, 0],
        });
    }

    let mut states = vec![HashMap::new(); 3];
    let mut inits = vec![0; 3];
    let mut periods: Vec<i64> = vec![0; 3];

    for i in 0..1000000 {
        if i == 1000 {
            let energy: i32 = moons
                .iter()
                .map(|m| {
                    m.pos.iter().map(|c| c.abs()).sum::<i32>()
                        * m.vel.iter().map(|c| c.abs()).sum::<i32>()
                })
                .sum();
            println!("First: {}", energy);
        }

        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                for coor in 0..3 {
                    if moons[i].pos[coor] > moons[j].pos[coor] {
                        moons[i].vel[coor] -= 1;
                        moons[j].vel[coor] += 1;
                    } else if moons[i].pos[coor] < moons[j].pos[coor] {
                        moons[i].vel[coor] += 1;
                        moons[j].vel[coor] -= 1;
                    }
                }
            }
        }

        for m in moons.iter_mut() {
            for coor in 0..3 {
                m.pos[coor] += m.vel[coor];
            }
        }

        for coor in 0..3 {
            if periods[coor] == 0 {
                let state: Vec<i32> = moons
                    .iter()
                    .flat_map(|m| vec![m.pos[coor], m.vel[coor]])
                    .collect();
                match states[coor].get(&state) {
                    Some(s) => {
                        inits[coor] = *s;
                        periods[coor] = i - *s;
                    }
                    None => {
                        states[coor].insert(state, i);
                    }
                }
            }
        }

        if !periods.iter().any(|&p| p == 0) {
            break;
        }
    }

    let repeat = inits.iter().max().unwrap() + lcm(lcm(periods[0], periods[1]), periods[2]);
    println!("Second: {}", repeat);
}
