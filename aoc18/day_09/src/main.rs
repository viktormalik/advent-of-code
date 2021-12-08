extern crate linked_list;

use linked_list::Cursor;
use linked_list::LinkedList;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input");

    let players: usize = input.split_whitespace().nth(0).unwrap().parse().unwrap();
    let largest: u32 = input.split_whitespace().nth(6).unwrap().parse().unwrap();

    println!("First: {}", play(players, largest));
    println!("Second: {}", play(players, largest * 100));
}

fn play(players: usize, largest: u32) -> u32 {
    let mut marbles: LinkedList<u32> = LinkedList::new();
    marbles.push_front(0);
    let mut current: Cursor<u32> = marbles.cursor();
    let mut players = vec![0; players];
    let mut p: usize = 0;

    for i in 1..=largest {
        if i % 23 == 0 {
            players[p] += i;
            let mut val: u32 = prev(&mut current);
            for _ in 0..7 {
                val = prev(&mut current);
            }
            players[p] += val;
            current.remove();
            next(&mut current);
        } else {
            next(&mut current);
            current.insert(i);
            next(&mut current);
        }
        p = (p + 1) % players.len();
    }
    *players.iter().max().unwrap()
}

fn next(c: &mut Cursor<u32>) -> u32 {
    match c.next() {
        Some(n) => return *n,
        None => {}
    };
    *c.next().unwrap()
}

fn prev(c: &mut Cursor<u32>) -> u32 {
    match c.prev() {
        Some(n) => return *n,
        None => {}
    };
    *c.prev().unwrap()
}
