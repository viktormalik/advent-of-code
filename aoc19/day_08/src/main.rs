use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let layers: Vec<Vec<u32>> = input
        .trim()
        .chars()
        .chunks(150)
        .into_iter()
        .map(|c| c.map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let min_layer = layers
        .iter()
        .min_by_key(|l| l.iter().filter(|&&x| x == 0).count())
        .unwrap();
    let check = min_layer.iter().filter(|&&x| x == 1).count()
        * min_layer.iter().filter(|&&x| x == 2).count();
    println!("First: {}", check);

    let picture: Vec<char> = { 0..150 }
        .map(|i| {
            match layers
                .iter()
                .map(|l| l[i])
                .find(|&x| x == 0 || x == 1)
                .unwrap()
            {
                1 => '#',
                _ => ' ',
            }
        })
        .collect();

    print!("Second:");
    for (i, p) in picture.iter().enumerate() {
        if i % 25 == 0 {
            println!();
        }
        print!("{}", p);
    }
}
