use std::fs;
use std::iter::Iterator;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let mut num = input.trim().split_whitespace();
    let mut sum = 0;
    let val = load_node(&mut num, &mut sum);
    println!("First: {}", sum);
    println!("Second: {}", val);
}

fn load_node<'a, I>(num: &mut I, sum: &mut usize) -> usize
where
    I: Iterator<Item = &'a str>,
{
    let child_cnt: usize = num.next().unwrap().parse().unwrap();
    let metadata_cnt: usize = num.next().unwrap().parse().unwrap();
    let mut children: Vec<usize> = Vec::new();
    let mut val = 0;
    for _ in 0..child_cnt {
        children.push(load_node(num, sum));
    }
    for _ in 0..metadata_cnt {
        let m: usize = num.next().unwrap().parse().unwrap();
        *sum += m;

        if child_cnt == 0 {
            val += m;
        } else if m <= children.len() {
            val += children[m - 1];
        }
    }
    val
}
