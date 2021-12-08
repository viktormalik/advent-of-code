use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn handle_portal(
    input_map: &Vec<Vec<char>>,
    first: (usize, usize),
    second: (usize, usize),
    entry: (usize, usize),
    outer: bool,
    portal_names: &mut HashMap<String, (usize, usize)>,
    inner_portals: &mut HashMap<(usize, usize), (usize, usize)>,
    outer_portals: &mut HashMap<(usize, usize), (usize, usize)>,
) {
    if input_map[first.0][first.1].is_uppercase()
        && input_map[second.0][second.1].is_uppercase()
        && input_map[entry.0][entry.1] == '.'
    {
        let portal_name: String = vec![input_map[first.0][first.1], input_map[second.0][second.1]]
            .iter()
            .collect();
        if portal_names.contains_key(&portal_name) {
            if outer {
                outer_portals.insert(entry, *portal_names.get(&portal_name).unwrap());
                inner_portals.insert(*portal_names.get(&portal_name).unwrap(), entry);
            } else {
                inner_portals.insert(entry, *portal_names.get(&portal_name).unwrap());
                outer_portals.insert(*portal_names.get(&portal_name).unwrap(), entry);
            }
        } else {
            portal_names.insert(portal_name, entry);
        }
    }
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

fn get_dst(
    map: &Vec<Vec<char>>,
    from: &(usize, usize, usize),
    to: &(usize, usize, usize),
    inner_portals: &HashMap<(usize, usize), (usize, usize)>,
    outer_portals: &HashMap<(usize, usize), (usize, usize)>,
    with_levels: bool,
) -> u32 {
    let mut dsts = HashMap::new();
    dsts.insert(*from, 0);

    let mut queue = VecDeque::new();
    queue.push_back(*from);

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        let current_dst = *dsts.get(&current).unwrap();

        if with_levels && current == *to {
            return current_dst;
        }

        if !with_levels && (current.0, current.1) == (to.0, to.1) {
            return current_dst;
        }

        let current_pos = (current.0, current.1);

        for dir in 0..4 {
            let mut next_pos = get_next(&current_pos, dir);
            let mut next_lvl = current.2;
            let next_char = map[next_pos.0][next_pos.1];
            if next_char == '#' {
                continue;
            }
            if next_char.is_uppercase() {
                if inner_portals.contains_key(&current_pos) {
                    next_pos = *inner_portals.get(&current_pos).unwrap();
                    next_lvl += 1;
                } else if outer_portals.contains_key(&current_pos) && next_lvl > 0 {
                    next_pos = *outer_portals.get(&current_pos).unwrap();
                    next_lvl -= 1;
                } else {
                    continue;
                }
            }
            let next = (next_pos.0, next_pos.1, next_lvl);
            let next_dst = current_dst + 1;
            if !dsts.contains_key(&next) {
                dsts.insert(next, next_dst);
                queue.push_back(next);
            }
        }
    }
    1000
}
fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let input_map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut portal_names = HashMap::new();
    let mut inner_portals = HashMap::new();
    let mut outer_portals = HashMap::new();

    for x in 0..input_map.len() - 2 {
        for y in 0..input_map[x].len() - 2 {
            handle_portal(
                &input_map,
                (x, y),
                (x + 1, y),
                (x + 2, y),
                x == 0,
                &mut portal_names,
                &mut inner_portals,
                &mut outer_portals,
            );
            handle_portal(
                &input_map,
                (x + 1, y),
                (x + 2, y),
                (x, y),
                x == input_map.len() - 3,
                &mut portal_names,
                &mut inner_portals,
                &mut outer_portals,
            );
            handle_portal(
                &input_map,
                (x, y),
                (x, y + 1),
                (x, y + 2),
                y == 0,
                &mut portal_names,
                &mut inner_portals,
                &mut outer_portals,
            );
            handle_portal(
                &input_map,
                (x, y + 1),
                (x, y + 2),
                (x, y),
                y == input_map[x].len() - 3,
                &mut portal_names,
                &mut inner_portals,
                &mut outer_portals,
            );
        }
    }
    let start = portal_names.get(&"AA".to_string()).unwrap();
    let end = portal_names.get(&"ZZ".to_string()).unwrap();

    let first = get_dst(
        &input_map,
        &(start.0, start.1, 0),
        &(end.0, end.1, 0),
        &inner_portals,
        &outer_portals,
        false,
    );
    println!("First: {}", first);

    let second = get_dst(
        &input_map,
        &(start.0, start.1, 0),
        &(end.0, end.1, 0),
        &inner_portals,
        &outer_portals,
        true,
    );
    println!("Second: {}", second);
}
