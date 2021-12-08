use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut clays: Vec<(usize, usize)> = Vec::new();
    for line in input.trim().lines() {
        let split: Vec<&str> = line.split(", ").collect();
        let left: Vec<&str> = split[0].split("=").collect();
        let right: Vec<&str> = split[1].split("=").collect();
        let fixed = left[1].parse::<usize>().unwrap();
        let range: Vec<usize> = right[1]
            .split("..")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for i in range[0]..range[1] + 1 {
            clays.push(match left[0] {
                "x" => (fixed, i),
                _ => (i, fixed),
            });
        }
    }

    let min_x = clays.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = clays.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = clays.iter().map(|(_, y)| y).min().unwrap();
    let max_y = clays.iter().map(|(_, y)| y).max().unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_x - min_x + 1]; max_y - min_y + 1];
    for (x, y) in &clays {
        grid[*y - min_y][*x - min_x] = '#';
    }

    flow(&mut grid, 500 - min_x, 0);

    let flowing: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&x| *x == '|').count())
        .sum();
    let filled: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&x| *x == '~').count())
        .sum();
    println!("First: {}", flowing + filled);
    println!("Second: {}", filled);
}

#[allow(dead_code)]
fn print(grid: &Vec<Vec<char>>) {
    println!();
    for (y, row) in grid.iter().enumerate() {
        print!("{0: <4}:", y);
        for x in row {
            print!("{}", x);
        }
        println!();
    }
}

fn flow(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    grid[y][x] = '|';

    if y == grid.len() - 1 {
        return true;
    }

    if grid[y + 1][x] == '|' {
        return true;
    }

    let down = grid[y + 1][x] == '.' && flow(grid, x, y + 1);

    if !down {
        let left = grid[y][x - 1] == '.' && flow(grid, x - 1, y);
        let right = grid[y][x + 1] == '.' && flow(grid, x + 1, y);
        if !left && !right {
            fill(grid, x, y);
            return false;
        } else {
            unfill(grid, x, y);
            return true;
        }
    }
    true
}

fn fill(grid: &mut Vec<Vec<char>>, x: usize, y: usize) {
    grid[y][x] = '~';
    if x > 0 && grid[y][x - 1] == '|' {
        fill(grid, x - 1, y);
    }
    if x < grid[y].len() - 1 && grid[y][x + 1] == '|' {
        fill(grid, x + 1, y);
    }
}

fn unfill(grid: &mut Vec<Vec<char>>, x: usize, y: usize) {
    grid[y][x] = '|';
    if x > 0 && grid[y][x - 1] == '~' {
        unfill(grid, x - 1, y);
    }
    if x < grid[y].len() - 1 && grid[y][x + 1] == '~' {
        unfill(grid, x + 1, y);
    }
}
