use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn get_points(line: &Vec<&str>) -> HashMap<Point, usize> {
    let mut points = HashMap::new();
    let mut current = Point { x: 0, y: 0 };
    let mut i = 0;

    for dir in line.iter() {
        let steps: usize = dir[1..].parse().unwrap();
        for _ in 0..steps {
            current.x += match dir.chars().nth(0).unwrap() {
                'R' => 1,
                'L' => -1,
                _ => 0,
            };
            current.y += match dir.chars().nth(0).unwrap() {
                'U' => 1,
                'D' => -1,
                _ => 0,
            };
            i += 1;
            points.insert(current, i);
        }
    }

    return points;
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let line1: Vec<&str> = input.lines().nth(0).unwrap().split(",").collect();
    let line2: Vec<&str> = input.lines().nth(1).unwrap().split(",").collect();

    let points1 = get_points(&line1);
    let points2 = get_points(&line2);

    let min_dst = points1
        .keys()
        .filter(|p| points2.contains_key(p))
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap();
    println!("First: {}", min_dst);

    let min_steps = points1
        .iter()
        .filter(|(p, _)| points2.contains_key(p))
        .map(|(p, d)| d + points2.get(p).unwrap())
        .min()
        .unwrap();
    println!("Second: {}", min_steps);
}
