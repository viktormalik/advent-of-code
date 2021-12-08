extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut all_ingredients: Vec<HashSet<String>> = vec![];

    // Mapping allergen -> list of potential ingredients
    let mut candidate_map: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.trim().lines() {
        let re = Regex::new(r"^(.*)\(contains(.*)\)$").unwrap();
        let caps = re.captures_iter(line).nth(0).unwrap();

        let ingredients: HashSet<String> = caps[1].split_whitespace().map(String::from).collect();
        all_ingredients.push(ingredients.clone());

        for a in caps[2].split(',').map(|a| a.trim()) {
            if !candidate_map.contains_key(a) {
                candidate_map.insert(a.to_string(), ingredients.clone());
            } else {
                candidate_map
                    .get_mut(a)
                    .unwrap()
                    .retain(|i| ingredients.contains(i));
            }
        }
    }

    // Set of all potential ingredients
    let candidate_ingreds: HashSet<String> = candidate_map
        .iter()
        .map(|(_, is)| is.iter().cloned())
        .flatten()
        .collect();

    let first: usize = all_ingredients
        .iter()
        .map(|is| {
            is.iter()
                .filter(|&i| !candidate_ingreds.contains(i))
                .count()
        })
        .sum();
    println!("First: {}", first);

    // Mapping allerg -> ingredient
    let mut allerg_map: HashMap<String, String> = HashMap::new();
    while !candidate_map.is_empty() {
        let allergen = candidate_map
            .iter()
            .filter(|(_, is)| is.len() == 1)
            .nth(0)
            .unwrap();

        let ingred = allergen.1.iter().nth(0).unwrap().clone();
        allerg_map.insert(allergen.0.clone(), ingred.clone());

        for (_, is) in candidate_map.iter_mut() {
            is.remove(&ingred);
        }
        candidate_map.retain(|_, is| !is.is_empty());
    }

    // Sort ingredients by their allergen
    let mut allergens_sorted: Vec<String> = allerg_map.keys().cloned().collect();
    allergens_sorted.sort();
    let ingredients_sorted: Vec<String> = allergens_sorted
        .iter()
        .map(|a| allerg_map.get(a).unwrap().clone())
        .collect();
    let second = ingredients_sorted.join(",");
    println!("Second: {}", second);
}
