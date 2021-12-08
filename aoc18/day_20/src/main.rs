extern crate petgraph;

use petgraph::algo::dijkstra;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut map = Graph::<(i32, i32), usize>::new();
    let start = map.add_node((0, 0));
    let mut current = start;
    let mut splits: Vec<NodeIndex> = Vec::new();

    for c in input.trim().chars() {
        let &(x, y) = map.node_weight(current).unwrap();
        match c {
            'N' | 'E' | 'S' | 'W' => {
                let next_point = match c {
                    'N' => (x, y + 1),
                    'E' => (x + 1, y),
                    'S' => (x, y - 1),
                    'W' => (x - 1, y),
                    _ => (x, y),
                };
                let next = match map
                    .node_indices()
                    .find(|&i| *map.node_weight(i).unwrap() == next_point)
                {
                    Some(i) => i,
                    None => map.add_node(next_point),
                };
                map.add_edge(current, next, 1);
                current = next;
            }
            '(' => {
                splits.push(current);
            }
            ')' => {
                splits.pop();
            }
            '|' => {
                current = *splits.last().unwrap();
            }
            _ => {}
        };
    }

    let dsts = dijkstra(&map, start, None, |_| 1);
    let max_dst = dsts.values().max().unwrap();
    println!("First: {}", max_dst);
    let count = dsts.values().filter(|&d| *d >= 1000).count();
    println!("Second: {}", count);
}
