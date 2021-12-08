use std::fs;

#[derive(Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn turn_left(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::W,
        Dir::S => Dir::E,
        Dir::E => Dir::N,
        Dir::W => Dir::S,
    }
}

fn turn_right(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::E,
        Dir::S => Dir::W,
        Dir::E => Dir::S,
        Dir::W => Dir::N,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut pos = (0, 0);
    let mut dir = Dir::E;

    for action in input.lines() {
        let a = action.as_bytes()[0] as char;
        let val: i32 = action[1..action.len()].parse().unwrap();
        if a == 'N' || a == 'F' && dir == Dir::N {
            pos = (pos.0 - val, pos.1)
        } else if a == 'S' || a == 'F' && dir == Dir::S {
            pos = (pos.0 + val, pos.1)
        } else if a == 'E' || a == 'F' && dir == Dir::E {
            pos = (pos.0, pos.1 + val)
        } else if a == 'W' || a == 'F' && dir == Dir::W {
            pos = (pos.0, pos.1 - val)
        }
        if action.as_bytes()[0] as char == 'L' {
            for _ in 0..(val / 90) {
                dir = turn_left(dir);
            }
        } else if action.as_bytes()[0] as char == 'R' {
            for _ in 0..(val / 90) {
                dir = turn_right(dir);
            }
        }
    }

    println!("First: {}", pos.0.abs() + pos.1.abs());

    let mut ship = (0, 0);
    let mut waypoint = (-1, 10);

    for action in input.lines() {
        let a = action.as_bytes()[0] as char;
        let val: i32 = action[1..action.len()].parse().unwrap();

        waypoint = match a {
            'N' => (waypoint.0 - val, waypoint.1),
            'S' => (waypoint.0 + val, waypoint.1),
            'E' => (waypoint.0, waypoint.1 + val),
            'W' => (waypoint.0, waypoint.1 - val),
            'L' => (0..val / 90).fold(waypoint, |pos, _| (-pos.1, pos.0)),
            'R' => (0..val / 90).fold(waypoint, |pos, _| (pos.1, -pos.0)),
            _ => waypoint,
        };

        if a == 'F' {
            ship = (ship.0 + val * waypoint.0, ship.1 + val * waypoint.1);
        }
    }
    println!("Second: {}", ship.0.abs() + ship.1.abs());
}
