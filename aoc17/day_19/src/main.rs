use std::fs;

#[derive(Eq, PartialEq)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let diagram: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut pos: (usize, usize) = (
        0,
        diagram[0]
            .iter()
            .enumerate()
            .find(|(_, &x)| x == '|')
            .unwrap()
            .0,
    );
    let mut dir = Dir::DOWN;

    let mut count = 1;
    print!("First: ");
    loop {
        if dir == Dir::DOWN || dir == Dir::UP {
            pos.0 = match dir {
                Dir::DOWN => pos.0 + 1,
                Dir::UP => pos.0 - 1,
                _ => pos.0,
            };
            if diagram[pos.0][pos.1] == '+' {
                if pos.1 > 0 && diagram[pos.0][pos.1 - 1] != ' ' {
                    dir = Dir::LEFT;
                } else {
                    dir = Dir::RIGHT;
                }
            }
        } else {
            pos.1 = match dir {
                Dir::RIGHT => pos.1 + 1,
                Dir::LEFT => pos.1 - 1,
                _ => pos.1,
            };
            if diagram[pos.0][pos.1] == '+' {
                if pos.0 > 0 && diagram[pos.0 - 1][pos.1] != ' ' {
                    dir = Dir::UP;
                } else {
                    dir = Dir::DOWN;
                }
            }
        }
        if diagram[pos.0][pos.1].is_uppercase() {
            print!("{}", diagram[pos.0][pos.1]);
        } else if diagram[pos.0][pos.1] == ' ' {
            break;
        }
        count += 1;
    }
    println!();
    println!("Second: {}", count);
}
