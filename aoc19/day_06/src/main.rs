use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

struct OrbitMap<'a>(HashMap<&'a str, &'a str>);

struct IterOrbitMap<'a> {
    map: &'a OrbitMap<'a>,
    current: &'a str,
}

impl<'a> Iterator for IterOrbitMap<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        match self.current {
            "COM" => None,
            o => {
                self.current = self.map.0.get(o).unwrap();
                Some(self.current)
            }
        }
    }
}

impl<'a> OrbitMap<'a> {
    fn path(&'a self, from: &'a str) -> IterOrbitMap<'a> {
        IterOrbitMap {
            map: self,
            current: from,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let orbit_map = OrbitMap(
        input
            .lines()
            .flat_map(|l| l.trim().split(")"))
            .tuples()
            .map(|(x, y)| (y, x))
            .collect(),
    );

    let orbits: usize = orbit_map.0.keys().map(|o| orbit_map.path(o).count()).sum();
    println!("First: {}", orbits);

    let min_transfers = orbit_map
        .path("YOU")
        .enumerate()
        .find_map(|(i, o)| match orbit_map.path("SAN").position(|p| p == o) {
            Some(j) => Some(i + j),
            None => None,
        })
        .unwrap();
    println!("Second: {}", min_transfers);
}
