extern crate num;

use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32;
use std::fs;

fn get_visible(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(i32, i32)> {
    let mut visible = vec![];
    for xother in 0..map.len() {
        for yother in 0..map[x].len() {
            if (xother == x && yother == y) || map[xother][yother] == '.' {
                continue;
            }
            let xshift: i32 = xother as i32 - x as i32;
            let yshift: i32 = yother as i32 - y as i32;
            let gcd = gcd(xshift, yshift);
            let xbase = xshift / gcd;
            let ybase = yshift / gcd;

            let mut xtest = (x as i32 + xbase) as usize;
            let mut ytest = (y as i32 + ybase) as usize;

            let mut is_visible = true;
            while !(xtest == xother && ytest == yother) {
                if map[xtest][ytest] == '#' {
                    is_visible = false;
                }
                xtest = (xtest as i32 + xbase) as usize;
                ytest = (ytest as i32 + ybase) as usize;
            }
            if is_visible {
                visible.push((xshift, yshift));
            }
        }
    }
    return visible;
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut visible_map = HashMap::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] != '.' {
                visible_map.insert((x, y), get_visible(&map, x, y));
            }
        }
    }

    let max_visible = visible_map.iter().max_by_key(|(_, v)| v.len()).unwrap();
    println!("First: {}", max_visible.1.len());

    let mut shifts = max_visible.1.clone();
    shifts.sort_by(|&(x1, y1), &(x2, y2)| {
        (y1 as f32)
            .atan2(x1 as f32)
            .partial_cmp(&(y2 as f32).atan2(x2 as f32))
            .unwrap_or(Ordering::Equal)
    });
    shifts.reverse();

    let resx = (max_visible.0).1 as i32 + shifts[199].1;
    let resy = (max_visible.0).0 as i32 + shifts[199].0;
    println!("Second: {}", resx * 100 + resy);
}
