use std::collections::HashSet;
use std::fs;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Pt {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut clusters: Vec<HashSet<Pt>> = Vec::new();

    for line in input.trim().lines() {
        let split: Vec<i32> = line
            .split(',')
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
        let new_pt = Pt {
            x: split[0],
            y: split[1],
            z: split[2],
            t: split[3],
        };

        let mut insert: Vec<usize> = Vec::new();
        for i in 0..clusters.len() {
            for p in &clusters[i] {
                if (new_pt.x - p.x).abs()
                    + (new_pt.y - p.y).abs()
                    + (new_pt.z - p.z).abs()
                    + (new_pt.t - p.t).abs()
                    <= 3
                {
                    insert.push(i);
                    break;
                }
            }
        }
        if insert.is_empty() {
            let mut new_cluster = HashSet::new();
            new_cluster.insert(new_pt);
            clusters.push(new_cluster);
        } else {
            clusters[insert[0]].insert(new_pt);
            let mut pts_to_insert = Vec::new();
            let mut clusters_to_remove: Vec<usize> = Vec::new();
            for i in insert.iter().skip(1) {
                for p in &clusters[*i] {
                    pts_to_insert.push(p.clone());
                }
                clusters_to_remove.push(*i);
            }
            clusters_to_remove.sort_by(|a, b| a.cmp(&b).reverse());

            for p in pts_to_insert {
                clusters[insert[0]].insert(p);
            }
            for c in clusters_to_remove {
                clusters.remove(c);
            }
        }
    }

    println!("First: {}", clusters.len());
}
