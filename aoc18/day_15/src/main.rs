use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Pt {
    x: usize,
    y: usize,
}

impl Pt {
    fn next(&self) -> Vec<Pt> {
        vec![
            Pt {
                x: self.x - 1,
                y: self.y,
            },
            Pt {
                x: self.x,
                y: self.y - 1,
            },
            Pt {
                x: self.x,
                y: self.y + 1,
            },
            Pt {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Kind {
    ELF,
    GOBLIN,
}

#[derive(Clone)]
struct Unit {
    pos: Pt,
    kind: Kind,
    hp: usize,
    power: usize,
}

fn minimal_distances(grid: &Vec<Vec<char>>, start: &Pt) -> Vec<Vec<(i32, Pt)>> {
    let mut dst: Vec<Vec<(i32, Pt)>> =
        vec![vec![(-1, Pt { x: 0, y: 0 }); grid[0].len()]; grid.len()];
    dst[start.x][start.y] = (0, Pt { x: 0, y: 0 });
    let mut to_explore: VecDeque<Pt> = VecDeque::new();
    // First step from the starting poisition - save taken direction
    for next in start.next() {
        if grid[next.x][next.y] != '#' {
            dst[next.x][next.y] = (1, next.clone());
            if grid[next.x][next.y] == '.' {
                to_explore.push_back(next);
            }
        }
    }
    while !to_explore.is_empty() {
        // Search all reachable squares
        let sq = to_explore.pop_front().unwrap();
        let d = dst[sq.x][sq.y].clone();
        for next in sq.next() {
            if grid[next.x][next.y] != '#' && dst[next.x][next.y].0 == -1 {
                dst[next.x][next.y] = (d.0 + 1, d.1.clone());
                if grid[next.x][next.y] == '.' {
                    to_explore.push_back(next);
                }
            }
        }
    }
    dst
}

#[allow(dead_code)]
fn print(round: usize, grid: &Vec<Vec<char>>, units: &Vec<Unit>) {
    println!("Round {}", round);
    for row in 0usize..grid.len() {
        for c in &grid[row] {
            print!("{}", c);
        }
        print!("  ");
        for u in units.iter().filter(|u| u.pos.x == row) {
            print!(
                "{}({}) ",
                match u.kind {
                    Kind::ELF => 'E',
                    Kind::GOBLIN => 'G',
                },
                u.hp
            );
        }
        println!();
    }
    println!();
}

fn fight(grid: &mut Vec<Vec<char>>, units: &mut Vec<Unit>) -> usize {
    let mut rounds = 0;
    loop {
        units.sort_by(|a, b| a.pos.cmp(&b.pos));
        let mut full_round = false;
        let mut dead: HashSet<usize> = HashSet::new();
        for i in 0..units.len() {
            if dead.contains(&i) {
                continue;
            }
            let dst = minimal_distances(&grid, &units[i].pos);
            // Find the closest unit (target)
            let target = { 0usize..units.len() }
                .filter(|&j| i != j)
                .filter(|&j| units[j].kind != units[i].kind)
                .filter(|&j| !dead.contains(&j))
                .filter(|&j| dst[units[j].pos.x][units[j].pos.y].0 > 0)
                .min_by(|&j, &k| {
                    dst[units[j].pos.x][units[j].pos.y]
                        .0
                        .cmp(&dst[units[k].pos.x][units[k].pos.y].0)
                });
            match target {
                Some(j) => {
                    // Move the current unit towards the target
                    let next = dst[units[j].pos.x][units[j].pos.y].1.clone();
                    if grid[next.x][next.y] == '.' {
                        grid[units[i].pos.x][units[i].pos.y] = '.';
                        units[i].pos = next;
                        grid[units[i].pos.x][units[i].pos.y] = match units[i].kind {
                            Kind::ELF => 'E',
                            Kind::GOBLIN => 'G',
                        };
                    }
                }
                None => {}
            }

            // Attack
            let mut enemies = Vec::new();
            for next in units[i].pos.next() {
                if grid[next.x][next.y]
                    == match units[i].kind {
                        Kind::ELF => 'G',
                        Kind::GOBLIN => 'E',
                    }
                {
                    match { 0usize..units.len() }
                        .filter(|&j| !dead.contains(&j))
                        .filter(|&j| units[j].pos == next)
                        .next()
                    {
                        Some(e) => enemies.push(e),
                        None => {}
                    }
                }
            }
            match enemies
                .iter()
                .min_by(|&a, &b| units[*a].hp.cmp(&units[*b].hp))
            {
                Some(e) => {
                    if units[*e].hp <= units[i].power {
                        dead.insert(*e);
                        grid[units[*e].pos.x][units[*e].pos.y] = '.';
                    } else {
                        units[*e].hp -= units[i].power;
                    }
                    if units
                        .iter()
                        .enumerate()
                        .skip(i + 1)
                        .all(|u| dead.contains(&u.0))
                    {
                        full_round = true;
                    }
                }
                None => {}
            }
        }
        let mut dead_vec: Vec<usize> = dead.iter().cloned().collect();
        dead_vec.sort_by(|a, b| a.cmp(&b).reverse());
        for d in dead_vec {
            units.remove(d);
        }

        if units.iter().all(|u| u.kind == Kind::ELF) || units.iter().all(|u| u.kind == Kind::GOBLIN)
        {
            let total_hp = units.iter().map(|u| u.hp).sum::<usize>();
            return total_hp * (rounds + full_round as usize);
        }
        rounds += 1;
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let grid_init: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let units_init: Vec<Unit> = grid_init
        .iter()
        .enumerate()
        .map::<Vec<Unit>, _>(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'E' || c == 'G')
                .map(|(y, c)| Unit {
                    pos: Pt { x: x, y: y },
                    kind: match c {
                        'E' => Kind::ELF,
                        _ => Kind::GOBLIN,
                    },
                    hp: 200,
                    power: 3,
                })
                .collect()
        })
        .flatten()
        .collect();

    let mut grid = grid_init.to_vec();
    let mut units = units_init.to_vec();
    let first = fight(&mut grid, &mut units);
    println!("First: {}", first);

    let elves_cnt = units_init.iter().filter(|u| u.kind == Kind::ELF).count();
    let mut loosing = 0;
    let mut winning = 200;
    let mut power = 100;
    let mut second = 0;

    while winning - loosing != 1 {
        let mut grid = grid_init.to_vec();
        let mut units = units_init.to_vec();
        for u in units.iter_mut().filter(|u| u.kind == Kind::ELF) {
            u.power = power;
        }

        let result = fight(&mut grid, &mut units);
        if units.iter().filter(|u| u.kind == Kind::ELF).count() == elves_cnt {
            // Elves win without looses
            second = result;
            winning = power;
            power = (power + loosing) / 2;
        } else {
            loosing = power;
            power = (power + winning) / 2;
        }
    }
    println!("Second: {}", second);
}
