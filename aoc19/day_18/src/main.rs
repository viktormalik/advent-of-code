use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Path {
    pos: Vec<(usize, usize)>,
    keys: BTreeSet<char>,
}

fn get_next(pos: &(usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        0 => (pos.0, pos.1 - 1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 - 1, pos.1),
        3 => (pos.0 + 1, pos.1),
        _ => *pos,
    }
}

fn reachable(
    map: &Vec<Vec<char>>,
    from: &(usize, usize),
    keys: &BTreeSet<char>,
) -> Vec<((usize, usize), u32)> {
    let mut dsts = HashMap::new();
    if !dsts.contains_key(from) {
        dsts.insert(*from, 0);
    }

    let mut queue = VecDeque::new();
    queue.push_back(*from);

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();

        if map[current.0][current.1].is_lowercase() && !keys.contains(&map[current.0][current.1]) {
            continue;
        }

        let current_dst = *dsts.get(&current).unwrap();

        for dir in 0..4 {
            let next = get_next(&current, dir);
            let next_char = map[next.0][next.1];
            if next_char == '#'
                || (next_char.is_uppercase()
                    && !keys.contains(&next_char.to_lowercase().nth(0).unwrap()))
            {
                continue;
            }
            let next_dst = current_dst + 1;
            if !dsts.contains_key(&next) {
                dsts.insert(next, next_dst);
                queue.push_back(next);
            }
        }
    }
    dsts.iter()
        .filter(|&(&(x, y), _)| map[x][y].is_lowercase() && !keys.contains(&map[x][y]))
        .map(|(&p, &d)| (p, d))
        .collect()
}

fn count_steps(map: &Vec<Vec<char>>, keys_cnt: usize) -> u32 {
    let mut current = vec![];
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '@' {
                current.push((x, y));
            }
        }
    }

    let mut paths = HashMap::new();
    let start = Path {
        pos: current,
        keys: BTreeSet::new(),
    };
    paths.insert(start.clone(), 0);

    let mut to_expand: VecDeque<Path> = VecDeque::new();
    to_expand.push_back(start);

    while to_expand.len() > 0 {
        let path = to_expand.pop_front().unwrap();
        let mut reachable_pts = HashMap::new();
        for i in 0..path.pos.len() {
            for r in reachable(&map, &path.pos[i], &path.keys) {
                reachable_pts.insert(r.0, (r.1, i));
            }
        }
        for r in reachable_pts {
            let mut new_keys = path.keys.clone();
            new_keys.insert(map[(r.0).0][(r.0).1]);
            let new_dist = paths.get(&path).unwrap() + (r.1).0;

            let mut new_pos = path.pos.clone();
            new_pos[(r.1).1] = r.0;

            let new_path = Path {
                pos: new_pos,
                keys: new_keys,
            };
            let new = paths.entry(new_path.clone()).or_insert(new_dist);

            if *new >= new_dist {
                *new = new_dist;
                if !to_expand.contains(&new_path) {
                    to_expand.push_back(new_path);
                }
            }
        }
    }

    *paths
        .iter()
        .filter(|&(p, _)| p.keys.len() == keys_cnt)
        .map(|(_, d)| d)
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let keys_cnt = input.chars().filter(|c| c.is_lowercase()).count();
    let mut map: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let first = count_steps(&map, keys_cnt);
    println!("First: {}", first);

    let mut start = (0, 0);
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '@' {
                start = (x, y);
            }
        }
    }

    map[start.0][start.1 + 1] = '#';
    map[start.0][start.1 - 1] = '#';
    map[start.0 + 1][start.1] = '#';
    map[start.0 - 1][start.1] = '#';
    map[start.0 + 1][start.1 + 1] = '@';
    map[start.0 + 1][start.1 - 1] = '@';
    map[start.0 - 1][start.1 + 1] = '@';
    map[start.0 - 1][start.1 - 1] = '@';

    let second = count_steps(&map, keys_cnt);
    println!("Second: {}", second);
}
