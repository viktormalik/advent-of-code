use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn calc_diversity(grid: &Vec<Vec<char>>) -> u32 {
    grid.iter()
        .flatten()
        .enumerate()
        .map(|(i, tile)| match tile {
            '#' => 2u32.pow(i as u32),
            _ => 0,
        })
        .sum()
}

fn neighbors_cnt(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    prev_grid: Option<&Vec<Vec<char>>>,
    next_grid: Option<&Vec<Vec<char>>>,
) -> usize {
    let mut neighbors = vec![];
    if prev_grid.is_some() {
        if x == 0 {
            neighbors.push(prev_grid.unwrap()[1][2]);
        } else if x == 4 {
            neighbors.push(prev_grid.unwrap()[3][2]);
        }
        if y == 0 {
            neighbors.push(prev_grid.unwrap()[2][1]);
        } else if y == 4 {
            neighbors.push(prev_grid.unwrap()[2][3]);
        }
    }
    if next_grid.is_some() {
        if x == 1 && y == 2 {
            for i in 0..5 {
                neighbors.push(next_grid.unwrap()[0][i]);
            }
        } else if x == 3 && y == 2 {
            for i in 0..5 {
                neighbors.push(next_grid.unwrap()[4][i]);
            }
        } else if x == 2 && y == 1 {
            for i in 0..5 {
                neighbors.push(next_grid.unwrap()[i][0]);
            }
        } else if x == 2 && y == 3 {
            for i in 0..5 {
                neighbors.push(next_grid.unwrap()[i][4]);
            }
        }
    }
    if x != 0 && !(x == 3 && y == 2 && next_grid.is_some()) {
        neighbors.push(grid[x - 1][y]);
    }
    if x != 4 && !(x == 1 && y == 2 && next_grid.is_some()) {
        neighbors.push(grid[x + 1][y]);
    }
    if y != 0 && !(x == 2 && y == 3 && next_grid.is_some()) {
        neighbors.push(grid[x][y - 1]);
    }
    if y != 4 && !(x == 2 && y == 1 && next_grid.is_some()) {
        neighbors.push(grid[x][y + 1]);
    }

    neighbors.iter().filter(|&n| *n == '#').count()
}

fn make_step(
    grid: &Vec<Vec<char>>,
    prev_grid: Option<&Vec<Vec<char>>>,
    next_grid: Option<&Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; grid[0].len()]; grid.len()];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if (next_grid.is_some() || prev_grid.is_some()) && x == 2 && y == 2 {
                continue;
            }
            let n = neighbors_cnt(x, y, grid, prev_grid, next_grid);
            result[x][y] = match (grid[x][y], n) {
                ('#', 1) => '#',
                ('#', _) => '.',
                ('.', 1) => '#',
                ('.', 2) => '#',
                (_, _) => '.',
            };
        }
    }
    result
}

fn is_empty(grid: &Vec<Vec<char>>) -> bool {
    grid.iter().flatten().all(|&tile| tile == '.')
}

fn _print(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for tile in row.iter() {
            print!("{}", tile);
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let init_grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let mut diversisies = HashSet::new();
    let mut grid = init_grid.clone();
    loop {
        let diversity = calc_diversity(&grid);
        if diversisies.contains(&diversity) {
            println!("First: {}", diversity);
            break;
        } else {
            diversisies.insert(diversity);
        }
        grid = make_step(&grid, None, None);
    }

    let mut grids = VecDeque::new();
    grids.push_front(init_grid);

    for _ in 0..200 {
        if !is_empty(grids.front().unwrap()) {
            grids.push_front(vec![vec!['.'; 5]; 5]);
        }
        if !is_empty(grids.back().unwrap()) {
            grids.push_back(vec![vec!['.'; 5]; 5]);
        }

        let mut new_grids = VecDeque::new();

        for i in 0..grids.len() {
            new_grids.push_back(make_step(
                &grids[i],
                match i > 0 {
                    true => grids.get(i - 1),
                    false => None,
                },
                grids.get(i + 1),
            ));
        }
        grids = new_grids;
    }
    let second = grids
        .iter()
        .flatten()
        .flatten()
        .filter(|&t| *t == '#')
        .count();
    println!("Second: {}", second);
}
