use std::fs;

struct Cart {
    x: i32,
    y: i32,
    dir: (i32, i32),
    next_turn: u32,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut carts: Vec<Cart> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c == '>' || c == '<' || c == 'v' || c == '^' {
                carts.push(Cart {
                    x: x as i32,
                    y: y as i32,
                    dir: match c {
                        '^' => (0, -1),
                        '>' => (1, 0),
                        'v' => (0, 1),
                        _ => (-1, 0),
                    },
                    next_turn: 0,
                });
            }
        }
    }
    sort_carts(&mut carts);

    let mut occupied: Vec<(i32, i32)> = carts.iter().map(|c| (c.x, c.y)).collect();

    let mut first = true;
    loop {
        let mut i: usize = 0;
        let mut to_remove: Vec<usize> = Vec::new();
        for c in carts.iter_mut() {
            if to_remove.iter().any(|&r| r == i) {
                continue;
            }
            c.x += c.dir.0;
            c.y += c.dir.1;
            let track = grid[c.y as usize][c.x as usize];
            if track == '/' || track == '\\' || track == '+' {
                turn(c, track);
            }
            match occupied
                .iter()
                .enumerate()
                .filter(|o| o.0 != i && (o.1).0 == c.x && (o.1).1 == c.y)
                .next()
            {
                Some(o) => {
                    if first {
                        println!("Crash: {},{}", c.x, c.y);
                        first = false;
                    }
                    to_remove.push(o.0);
                    to_remove.push(i);
                }
                None => {}
            };
            if !to_remove.iter().any(|&r| r == i) {
                occupied[i] = (c.x, c.y);
            }
            i += 1;
        }
        to_remove.sort_by(|a, b| a.cmp(&b).reverse());
        for r in to_remove {
            carts.remove(r);
        }
        if carts.len() == 1 {
            println!("Second: {},{}", carts[0].x, carts[0].y);
            return;
        }
        occupied = carts.iter().map(|c| (c.x, c.y)).collect();
        sort_carts(&mut carts);
    }
}

fn sort_carts(carts: &mut Vec<Cart>) {
    carts.sort_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)));
}

fn turn(c: &mut Cart, track: char) {
    c.dir = match track {
        '/' => (-c.dir.1, -c.dir.0),
        '\\' => (c.dir.1, c.dir.0),
        '+' => match c.next_turn {
            0 => (c.dir.1, -c.dir.0),
            2 => (-c.dir.1, c.dir.0),
            _ => c.dir,
        },
        _ => c.dir,
    };
    if track == '+' {
        c.next_turn = (c.next_turn + 1) % 3;
    }
}
