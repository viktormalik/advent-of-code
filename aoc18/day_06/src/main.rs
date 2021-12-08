use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut pts: HashSet<(usize, usize)> = HashSet::new();
    for line in input.trim().lines() {
        let split: Vec<_> = line.split(',').collect();
        let x = split[0].trim().parse::<usize>().unwrap();
        let y = split[1].trim().parse::<usize>().unwrap();
        pts.insert((x, y));
    }

    let min_x = pts.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = pts.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = pts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = pts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let mut cnt = 0;
    for i in 0..max_x - min_x + 1 {
        for j in 0..max_y - min_y + 1 {
            let mut dst = 0;
            for (xp, yp) in &pts {
                dst += dist(xp - min_x, i, yp - min_y, j);
            }
            if dst < 10000 {
                cnt += 1;
            }
        }
    }
    println!("Second: {}", cnt);

    let mut grid = vec![vec![0; max_y - min_y + 1]; max_x - min_x + 1];
    let mut map = HashMap::new();
    let mut top = 1;
    for (x, y) in &pts {
        grid[x - min_x][y - min_y] = top;
        top += 1;
        map.insert(top, 0);
    }

    loop {
        let mut next_pts: HashSet<(usize, usize)> = HashSet::new();
        for (xp, yp) in &pts {
            let val = grid[xp - min_x][yp - min_y];
            if xp < &max_x {
                propagate(
                    &mut grid,
                    xp - min_x + 1,
                    yp - min_y,
                    val,
                    top,
                    &mut next_pts,
                );
            }
            if xp > &min_x {
                propagate(
                    &mut grid,
                    xp - min_x - 1,
                    yp - min_y,
                    val,
                    top,
                    &mut next_pts,
                );
            }
            if yp < &max_y {
                propagate(
                    &mut grid,
                    xp - min_x,
                    yp - min_y + 1,
                    val,
                    top,
                    &mut next_pts,
                );
            }
            if yp > &min_y {
                propagate(
                    &mut grid,
                    xp - min_x,
                    yp - min_y - 1,
                    val,
                    top,
                    &mut next_pts,
                );
            }
        }
        if next_pts.is_empty() {
            break;
        }
        pts.clear();
        pts = HashSet::from_iter(next_pts.iter().map(|(x, y)| (x + min_x, y + min_y)));
    }

    for x in 0..max_x - min_x + 1 {
        for y in 0..max_y - min_y + 1 {
            let val = grid[x][y];
            if x == 0 || x == max_x - min_x || y == 0 || y == max_y - min_y {
                map.remove(&val);
            } else {
                map.entry(val).and_modify(|cnt| *cnt += 1);
            }
        }
    }

    println!("First: {}", map.values().max_by(|a, b| a.cmp(&b)).unwrap());
}

fn dist(x1: usize, x2: usize, y1: usize, y2: usize) -> i32 {
    ((x1 as i32) - (x2 as i32)).abs() + ((y1 as i32) - (y2 as i32)).abs()
}

fn propagate(
    grid: &mut Vec<Vec<usize>>,
    x: usize,
    y: usize,
    val: usize,
    top: usize,
    new_pts: &mut HashSet<(usize, usize)>,
) {
    if grid[x][y] > 0 {
        if grid[x][y] != val && new_pts.contains(&(x, y)) {
            grid[x][y] = top;
        }
    } else {
        grid[x][y] = val;
        new_pts.insert((x, y));
    }
}
