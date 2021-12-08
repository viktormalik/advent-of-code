extern crate petgraph;
extern crate regex;

use petgraph::prelude::NodeIndex;
use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;
use petgraph::Graph;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut graph = Graph::<char, usize>::new();
    let mut nodes: HashMap<char, NodeIndex> = HashMap::new();

    // Create the graph
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin\.").unwrap();
    for line in input.trim().lines() {
        let cap = re.captures(line).unwrap();

        let node1 = cap[1].chars().next().unwrap();
        let node2 = cap[2].chars().next().unwrap();
        let i1 = match nodes.entry(node1) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => *e.insert(graph.add_node(node1)),
        };
        let i2 = match nodes.entry(node2) {
            Entry::Occupied(mut e) => *e.get_mut(),
            Entry::Vacant(e) => *e.insert(graph.add_node(node2)),
        };
        graph.add_edge(i1, i2, 1);
    }

    let node = graph
        .node_indices()
        .find(|i| graph.neighbors_directed(*i, Incoming).count() == 0)
        .unwrap();

    print!("First: ");
    let mut workqueue: VecDeque<NodeIndex> = VecDeque::new();
    workqueue.push_back(node);
    while !workqueue.is_empty() {
        let n = workqueue.pop_front().unwrap();
        let letter = graph.node_weight(n).unwrap();

        if nodes.remove(letter).is_some() {
            print!("{}", letter);
            for o in graph.neighbors_directed(n, Outgoing) {
                let can_process = graph
                    .neighbors_directed(o, Incoming)
                    .filter(|&i| {
                        workqueue.contains(&i) || nodes.contains_key(graph.node_weight(i).unwrap())
                    })
                    .next()
                    .is_none();
                if can_process {
                    workqueue.push_back(o);
                }
            }
        }
        let mut vec = Vec::from(workqueue);
        vec.sort_by(|a, b| graph.node_weight(*a).cmp(&graph.node_weight(*b)));
        workqueue = VecDeque::from(vec);
    }
    println!();

    for i in graph.node_indices() {
        nodes.insert(*graph.node_weight(i).unwrap(), i);
    }

    print!("Second: ");
    let mut workers = vec![(0, NodeIndex::new(100)); 5];
    let mut time: u32 = 0;
    let mut workqueue: VecDeque<(NodeIndex, u32)> = VecDeque::new();
    workqueue.push_back((node, 0));
    nodes.remove(graph.node_weight(node).unwrap());
    loop {
        let mut new_nodes: Vec<(NodeIndex, u32)> = Vec::new();
        for w in workers.iter_mut().filter(|w| w.0 == 0) {
            let n = workqueue.pop_front();
            if n.is_some() {
                if n.unwrap().1 <= time {
                    let letter = graph.node_weight(n.unwrap().0).unwrap();
                    *w = ((*letter as u32) - 4, n.unwrap().0);

                    for o in graph.neighbors_directed(n.unwrap().0, Outgoing) {
                        let can_process = graph
                            .neighbors_directed(o, Incoming)
                            .filter(|&i| {
                                workqueue.iter().find(|n| n.0 == i).is_some()
                                    || nodes.contains_key(graph.node_weight(i).unwrap())
                            })
                            .next()
                            .is_none();
                        if can_process {
                            nodes.remove(graph.node_weight(o).unwrap());
                            workqueue.push_back((o, time + w.0));
                            new_nodes.push((o, time + w.0));
                        }
                    }
                } else {
                    workqueue.push_front(n.unwrap());
                }
            }
        }

        for n in new_nodes {
            for i in graph.neighbors_directed(n.0, Incoming) {
                let i_worker = workers.iter().find(|w| w.1 == i);
                if i_worker.is_some() {
                    if i_worker.unwrap().0 > n.1 {
                        workqueue.iter_mut().find(|w| w.0 == n.0).unwrap().1 = i_worker.unwrap().0;
                    }
                }
            }
        }

        let time_shift = workers.iter().filter(|&w| w.0 > 0).min().unwrap().0;
        time += time_shift;
        for i in 0..workers.len() {
            if workers[i].0 > 0 {
                workers[i].0 -= time_shift;
            }
        }
        if nodes.is_empty() && workqueue.is_empty() {
            break;
        }
        let mut vec = Vec::from(workqueue);
        vec.sort_by(|a, b| (a.1, graph.node_weight(a.0)).cmp(&(b.1, graph.node_weight(b.0))));
        workqueue = VecDeque::from(vec);
    }
    time += workers.iter().max().unwrap().0;
    println!("{}", time);
}
