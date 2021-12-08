use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Component {
    id: u32,
    x: u32,
    y: u32,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Path {
    comps: Vec<Component>,
    last: u32,
}

fn find_paths(
    current_path: &Path,
    all_paths: &mut HashSet<Path>,
    components: &HashMap<u32, HashSet<Component>>,
) {
    for comp in components.get(&current_path.last).unwrap() {
        if !current_path.comps.iter().any(|c| c.id == comp.id) {
            let mut new_path = current_path.clone();
            new_path.comps.push(comp.clone());
            if current_path.last == comp.x {
                new_path.last = comp.y;
            } else {
                new_path.last = comp.x;
            }

            all_paths.insert(new_path.clone());

            find_paths(&new_path, all_paths, components);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut components: HashMap<u32, HashSet<Component>> = HashMap::new();
    let mut i = 0;

    for line in input.trim().lines() {
        let s: Vec<&str> = line.split('/').collect();
        let comp = Component {
            id: i,
            x: s[0].parse().unwrap(),
            y: s[1].parse().unwrap(),
        };
        components
            .entry(comp.x)
            .or_insert(HashSet::new())
            .insert(comp.clone());
        components
            .entry(comp.y)
            .or_insert(HashSet::new())
            .insert(comp);
        i += 1;
    }

    let mut paths: HashSet<Path> = HashSet::new();
    let start_path = Path {
        comps: vec![],
        last: 0,
    };
    find_paths(&start_path, &mut paths, &components);

    let first: u32 = paths
        .iter()
        .map(|p| p.comps.iter().map(|c| c.x + c.y).sum())
        .max()
        .unwrap();
    println!("First: {}", first);

    let max_len = paths.iter().map(|p| p.comps.len()).max().unwrap();
    let second: u32 = paths
        .iter()
        .filter_map(|p| match p.comps.len() == max_len {
            true => Some(p.comps.iter().map(|c| c.x + c.y).sum()),
            false => None,
        })
        .max()
        .unwrap();
    println!("Second: {}", second);
}
