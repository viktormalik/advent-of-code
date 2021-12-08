extern crate regex;

use regex::Regex;
use std::fs;

struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");
    let mut points: Vec<Point> = Vec::new();

    let re = Regex::new(r"^position=<(.*),(.*)> velocity=<(.*),(.*)>$").unwrap();
    for line in input.trim().lines() {
        let cap = re.captures(line).unwrap();
        points.push(Point {
            x: cap[1].trim().parse().unwrap(),
            y: cap[2].trim().parse().unwrap(),
            vx: cap[3].trim().parse().unwrap(),
            vy: cap[4].trim().parse().unwrap(),
        });
    }

    let mut size_prev = get_size(&points);
    let mut sec = 0;
    loop {
        for p in points.iter_mut() {
            p.x += p.vx;
            p.y += p.vy
        }
        let size = get_size(&points);
        if size.0 > size_prev.0 || size.1 > size_prev.1 {
            for p in points.iter_mut() {
                p.x -= p.vx;
                p.y -= p.vy;
            }
            break;
        }
        sec += 1;
        size_prev = size;
    }
    println!("First:");
    print(&points);
    println!("Second: {}", sec);
}

fn get_size(points: &Vec<Point>) -> (usize, usize) {
    let min_x = points.iter().min_by(|p, q| p.x.cmp(&q.x)).unwrap().x;
    let max_x = points.iter().max_by(|p, q| p.x.cmp(&q.x)).unwrap().x;
    let min_y = points.iter().min_by(|p, q| p.y.cmp(&q.y)).unwrap().y;
    let max_y = points.iter().max_by(|p, q| p.y.cmp(&q.y)).unwrap().y;

    let xrange: usize = (max_x - min_x + 1) as usize;
    let yrange: usize = (max_y - min_y + 1) as usize;
    (xrange, yrange)
}

fn print(points: &Vec<Point>) {
    let min_x = points.iter().min_by(|p, q| p.x.cmp(&q.x)).unwrap().x;
    let max_x = points.iter().max_by(|p, q| p.x.cmp(&q.x)).unwrap().x;
    let min_y = points.iter().min_by(|p, q| p.y.cmp(&q.y)).unwrap().y;
    let max_y = points.iter().max_by(|p, q| p.y.cmp(&q.y)).unwrap().y;

    let xrange: usize = (max_x - min_x + 1) as usize;
    let yrange: usize = (max_y - min_y + 1) as usize;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; xrange]; yrange];

    for p in points {
        grid[(p.y - min_y) as usize][(p.x - min_x) as usize] = '#';
    }
    for j in 0usize..yrange {
        for i in 0usize..xrange {
            print!("{}", grid[j][i]);
        }
        println!();
    }
}
