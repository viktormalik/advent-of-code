use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Clone, Eq, PartialEq)]
enum Color {
    Black,
    White,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn next(pos: &Pos, dir: Dir) -> Pos {
    Pos {
        x: match dir {
            Dir::E => pos.x + 1,
            Dir::W => pos.x - 1,
            Dir::NE | Dir::SE => match pos.y % 2 {
                0 => pos.x,
                _ => pos.x + 1,
            },
            Dir::NW | Dir::SW => match pos.y % 2 {
                0 => pos.x - 1,
                _ => pos.x,
            },
        },
        y: match dir {
            Dir::E | Dir::W => pos.y,
            Dir::NE | Dir::NW => pos.y + 1,
            Dir::SE | Dir::SW => pos.y - 1,
        },
    }
}

fn find_tile(dirs: &Vec<Dir>) -> Pos {
    dirs.iter()
        .fold(Pos { x: 0, y: 0 }, |pos, dir| next(&pos, dir.clone()))
}

fn parse_move(m: &str) -> Vec<Dir> {
    let mut res = vec![];

    let mut it = m.chars();
    while let Some(c) = it.next() {
        let dir = match c {
            'e' => Dir::E,
            'w' => Dir::W,
            's' => match it.next() {
                Some('e') => Dir::SE,
                Some('w') => Dir::SW,
                _ => Dir::E,
            },
            'n' => match it.next() {
                Some('e') => Dir::NE,
                Some('w') => Dir::NW,
                _ => Dir::E,
            },
            _ => Dir::E,
        };
        res.push(dir);
    }
    res
}

fn flip(pos: &Pos, tiles: &mut HashMap<Pos, Color>) {
    let tile = tiles.entry(pos.clone()).or_insert(Color::White);
    *tile = match *tile {
        Color::Black => Color::White,
        Color::White => Color::Black,
    };
}

fn init_state(moves: &Vec<&str>) -> HashMap<Pos, Color> {
    let mut tiles: HashMap<Pos, Color> = HashMap::new();

    for m in moves {
        let dirs = parse_move(m);
        let tile = find_tile(&dirs);
        flip(&tile, &mut tiles);
    }

    tiles
}

fn surr_black(tile: &Pos, tiles: &mut HashMap<Pos, Color>) -> usize {
    [Dir::E, Dir::W, Dir::SE, Dir::SW, Dir::NE, Dir::NW]
        .iter()
        .filter(|&dir| {
            *tiles.entry(next(tile, dir.clone())).or_insert(Color::White) == Color::Black
        })
        .count()
}

fn daily_flip(tiles: &mut HashMap<Pos, Color>) -> HashMap<Pos, Color> {
    let mut res_tiles = HashMap::new();

    let min_x = tiles.iter().map(|(pos, _)| pos.x).min().unwrap();
    let max_x = tiles.iter().map(|(pos, _)| pos.x).max().unwrap();
    let min_y = tiles.iter().map(|(pos, _)| pos.y).min().unwrap();
    let max_y = tiles.iter().map(|(pos, _)| pos.y).max().unwrap();

    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            let tile = Pos { x: x, y: y };
            let mut color = tiles.entry(tile.clone()).or_insert(Color::White).clone();

            let blacks = surr_black(&tile, tiles);

            if color == Color::Black && (blacks == 0 || blacks > 2) {
                color = Color::White;
            } else if color == Color::White && blacks == 2 {
                color = Color::Black;
            }
            res_tiles.insert(tile, color);
        }
    }

    res_tiles
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let moves = input.trim().lines().collect();

    let mut tiles = init_state(&moves);

    let first = tiles.iter().filter(|(_, t)| t == &&Color::Black).count();
    println!("First: {}", first);

    for _ in 0..100 {
        tiles = daily_flip(&mut tiles);
    }

    let second = tiles.iter().filter(|(_, t)| t == &&Color::Black).count();
    println!("Second: {}", second);
}
