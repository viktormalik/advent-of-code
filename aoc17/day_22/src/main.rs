use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    CLEAN,
    WEAK,
    INFECT,
    FLAG,
}

fn turn_left(d: Dir) -> Dir {
    match d {
        Dir::UP => Dir::LEFT,
        Dir::LEFT => Dir::DOWN,
        Dir::DOWN => Dir::RIGHT,
        Dir::RIGHT => Dir::UP,
    }
}

fn turn_right(d: Dir) -> Dir {
    match d {
        Dir::UP => Dir::RIGHT,
        Dir::RIGHT => Dir::DOWN,
        Dir::DOWN => Dir::LEFT,
        Dir::LEFT => Dir::UP,
    }
}

fn reverse(d: Dir) -> Dir {
    match d {
        Dir::UP => Dir::DOWN,
        Dir::DOWN => Dir::UP,
        Dir::RIGHT => Dir::LEFT,
        Dir::LEFT => Dir::RIGHT,
    }
}

fn next_dir(d: Dir, s: State) -> Dir {
    match s {
        State::CLEAN => turn_left(d),
        State::WEAK => d,
        State::INFECT => turn_right(d),
        State::FLAG => reverse(d),
    }
}

fn next(pos: (i32, i32), d: Dir) -> (i32, i32) {
    match d {
        Dir::UP => (pos.0 - 1, pos.1),
        Dir::DOWN => (pos.0 + 1, pos.1),
        Dir::LEFT => (pos.0, pos.1 - 1),
        Dir::RIGHT => (pos.0, pos.1 + 1),
    }
}

fn new_state(s: State, simple: bool) -> State {
    match s {
        State::CLEAN => match simple {
            true => State::INFECT,
            false => State::WEAK,
        },
        State::WEAK => State::INFECT,
        State::INFECT => match simple {
            true => State::CLEAN,
            false => State::FLAG,
        },
        State::FLAG => State::CLEAN,
    }
}

fn burst(
    pos: &mut (i32, i32),
    dir: &mut Dir,
    grid: &mut HashMap<(i32, i32), State>,
    simple: bool,
) -> State {
    let state = grid.entry(*pos).or_insert(State::CLEAN);
    *dir = next_dir(*dir, *state);
    *state = new_state(*state, simple);
    *pos = next(*pos, *dir);
    *state
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let start_grid: HashMap<(i32, i32), State> = input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, x)| *x == '#')
                .map(move |(c, _)| ((r as i32, c as i32), State::INFECT))
        })
        .flatten()
        .collect();

    let middle = input.lines().count() / 2;

    let mut grid = start_grid.clone();
    let mut pos: (i32, i32) = (middle as i32, middle as i32);
    let mut dir = Dir::UP;

    let first = (0..10000)
        .map(|_| burst(&mut pos, &mut dir, &mut grid, true))
        .filter(|s| *s == State::INFECT)
        .count();
    println!("First: {}", first);

    let mut grid = start_grid.clone();
    let mut pos: (i32, i32) = (middle as i32, middle as i32);
    let mut dir = Dir::UP;

    let second = (0..10000000)
        .map(|_| burst(&mut pos, &mut dir, &mut grid, false))
        .filter(|s| *s == State::INFECT)
        .count();
    println!("Second: {}", second);
}
