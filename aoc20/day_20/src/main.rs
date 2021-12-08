use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone)]
enum Side {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

fn parse(tile: &str) -> (u32, Vec<Vec<char>>) {
    (
        tile.lines().nth(0).unwrap()[5..9].parse().unwrap(),
        tile.lines()
            .skip(1)
            .map(|line| line.chars().collect())
            .collect(),
    )
}

fn side(tile: &Vec<Vec<char>>, s: Side) -> Vec<char> {
    match s {
        Side::TOP => tile[0].clone(),
        Side::BOTTOM => tile[tile.len() - 1].clone(),
        Side::LEFT => tile.iter().map(|r| r[0]).collect(),
        Side::RIGHT => tile.iter().map(|r| r[r.len() - 1]).collect(),
    }
}

fn sides(tile: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    vec![
        side(tile, Side::TOP),
        side(tile, Side::BOTTOM),
        side(tile, Side::LEFT),
        side(tile, Side::RIGHT),
    ]
}

fn rotate(tile: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![vec!['.'; tile.len()]; tile.len()];
    for r in 0..tile.len() {
        for c in 0..tile.len() {
            res[r][c] = tile[c][tile.len() - 1 - r];
        }
    }
    res
}

fn flip(tile: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![vec!['.'; tile.len()]; tile.len()];
    for r in 0..tile.len() {
        for c in 0..tile.len() {
            res[r][c] = tile[r][tile.len() - 1 - c];
        }
    }
    res
}

fn fit(tile: &Vec<Vec<char>>, match_side: (Side, Vec<char>)) -> Option<Vec<Vec<char>>> {
    let mut result = tile.clone();
    for _ in 0..4 {
        if side(&result, match_side.0) == match_side.1 {
            return Some(result);
        }
        result = rotate(&result);
    }
    result = flip(&result);
    for _ in 0..4 {
        if side(&result, match_side.0) == match_side.1 {
            return Some(result);
        }
        result = rotate(&result);
    }
    return None;
}

#[allow(dead_code)]
fn print_tile(tile: &Vec<Vec<char>>) {
    for r in 0..tile.len() {
        for c in 0..tile[r].len() {
            print!("{}", tile[r][c]);
        }
        println!();
    }
    println!();
}

fn mark_monsters(image: &mut Vec<Vec<char>>) -> u32 {
    let monster: Vec<Vec<char>> = vec![
        "                  # ".chars().collect(),
        "#    ##    ##    ###".chars().collect(),
        " #  #  #  #  #  #   ".chars().collect(),
    ];

    let mut count = 0;
    for r in 0..image.len() - monster.len() {
        for c in 0..image[r].len() - monster[0].len() {
            if monster.iter().enumerate().all(|(mr, row)| {
                row.iter()
                    .enumerate()
                    .all(|(mc, &cell)| cell == ' ' || image[r + mr][c + mc] == '#')
            }) {
                count += 1;
                for mr in 0..monster.len() {
                    for mc in 0..monster[0].len() {
                        if monster[mr][mc] == '#' {
                            image[r + mr][c + mc] = 'O';
                        }
                    }
                }
            }
        }
    }
    count
}

fn find_monsters(image: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        if mark_monsters(image) > 0 {
            return;
        }
        *image = rotate(image);
    }
    *image = flip(image);
    for _ in 0..4 {
        if mark_monsters(image) > 0 {
            return;
        }
        *image = rotate(image);
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let tiles: HashMap<u32, Vec<Vec<char>>> = input.split("\n\n").map(|tile| parse(tile)).collect();
    let mut matches: HashMap<u32, Vec<u32>> = HashMap::new();
    for (id, tile) in &tiles {
        for (other_id, other_tile) in &tiles {
            if id == other_id {
                continue;
            }
            if sides(tile).iter().any(|side| {
                sides(other_tile).iter().any(|other_side| {
                    side.iter().eq(other_side.iter())
                        || side.iter().rev().eq(other_side.iter())
                        || side.iter().eq(other_side.iter().rev())
                        || side.iter().rev().eq(other_side.iter().rev())
                })
            }) {
                matches.entry(*id).or_insert(vec![]).push(*other_id);
            }
        }
    }

    let first: u64 = matches
        .iter()
        .filter(|(_, m)| m.len() == 2)
        .fold(1, |acc, (id, _)| acc * *id as u64);
    println!("First: {}", first);

    let tile_size = tiles.values().nth(0).unwrap().len();
    let image_dim = (tiles.len() as f32).sqrt() as usize;

    let mut image = vec![vec!['.'; image_dim * tile_size]; image_dim * tile_size];
    let mut start_r = 0;
    let mut start_c = 0;

    let mut next_id = matches
        .iter()
        .find(|(_, m)| m.len() == 2)
        .unwrap()
        .0
        .clone();
    let mut next_tile = tiles.get(&next_id).unwrap().clone();

    // Proper rotation of the starting tile
    while !(matches.get(&next_id).unwrap().iter().any(|m| {
        fit(
            tiles.get(m).unwrap(),
            (Side::LEFT, side(&next_tile, Side::RIGHT)),
        )
        .is_some()
    }) && matches.get(&next_id).unwrap().iter().any(|m| {
        fit(
            tiles.get(m).unwrap(),
            (Side::TOP, side(&next_tile, Side::BOTTOM)),
        )
        .is_some()
    })) {
        next_tile = rotate(&next_tile);
    }

    let mut line_start_id = next_id;
    let mut line_start_tile = next_tile.clone();

    for _ in 0..image_dim {
        for j in 0..image_dim {
            // Copy current tile
            for r in 0..tile_size {
                for c in 0..tile_size {
                    image[start_r + r][start_c + c] = next_tile[r][c];
                }
            }
            // Get next tile
            if j != image_dim - 1 {
                for other in matches.get(&next_id).unwrap() {
                    let candidate = fit(
                        tiles.get(other).unwrap(),
                        (Side::LEFT, side(&next_tile, Side::RIGHT)),
                    );
                    if candidate.is_some() {
                        next_id = *other;
                        next_tile = candidate.unwrap();
                        break;
                    }
                }
            }
            start_c += tile_size;
        }
        // Get first tile of a row
        for other in matches.get(&line_start_id).unwrap() {
            let candidate = fit(
                tiles.get(other).unwrap(),
                (Side::TOP, side(&line_start_tile, Side::BOTTOM)),
            );
            if candidate.is_some() {
                next_id = *other;
                line_start_id = next_id;
                next_tile = candidate.unwrap();
                line_start_tile = next_tile.clone();
                break;
            }
        }
        start_r += tile_size;
        start_c = 0;
    }

    for r in 0..image.len() {
        if r % 10 == 0 || r % 10 == 9 {
            image[r] = vec![];
        } else {
            for c in 0..image[r].len() {
                if c % 10 == 0 || c % 10 == 9 {
                    image[r][c] = ' ';
                }
            }
            image[r] = image[r].iter().filter(|&c| *c != ' ').cloned().collect();
        }
    }
    image = image.iter().filter(|row| row.len() > 0).cloned().collect();

    find_monsters(&mut image);

    let second: usize = image
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum();
    println!("Second: {}", second);
}
