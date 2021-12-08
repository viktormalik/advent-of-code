use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

enum Player {
    P1,
    P2,
}

fn merge_decks(p1_deck: &VecDeque<usize>, p2_deck: &VecDeque<usize>) -> Vec<usize> {
    p1_deck
        .iter()
        .chain([0].iter())
        .chain(p2_deck.iter())
        .cloned()
        .collect()
}

fn round(p1_deck: &mut VecDeque<usize>, p2_deck: &mut VecDeque<usize>, recursive: bool) {
    let card1 = p1_deck.pop_front().unwrap();
    let card2 = p2_deck.pop_front().unwrap();

    let winner;

    if recursive && card1 <= p1_deck.len() && card2 <= p2_deck.len() {
        let mut p1_new_deck: VecDeque<usize> = p1_deck.iter().take(card1).cloned().collect();
        let mut p2_new_deck: VecDeque<usize> = p2_deck.iter().take(card2).cloned().collect();
        winner = game(&mut p1_new_deck, &mut p2_new_deck, recursive);
    } else {
        winner = match card1 > card2 {
            true => Player::P1,
            false => Player::P2,
        };
    }

    match winner {
        Player::P1 => {
            p1_deck.push_back(card1);
            p1_deck.push_back(card2);
        }
        Player::P2 => {
            p2_deck.push_back(card2);
            p2_deck.push_back(card1);
        }
    }
}

fn game(p1_deck: &mut VecDeque<usize>, p2_deck: &mut VecDeque<usize>, recursive: bool) -> Player {
    let mut games_cache: HashSet<Vec<usize>> = HashSet::new();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let decks = merge_decks(p1_deck, p2_deck);
        if recursive && games_cache.contains(&decks) {
            return Player::P1;
        }
        games_cache.insert(decks);

        round(p1_deck, p2_deck, recursive);
    }

    match p1_deck.is_empty() {
        true => Player::P2,
        false => Player::P1,
    }
}

fn score(winner: Player, p1_deck: &VecDeque<usize>, p2_deck: &VecDeque<usize>) -> usize {
    let winner_deck = match winner {
        Player::P1 => p1_deck,
        Player::P2 => p2_deck,
    };
    winner_deck
        .iter()
        .enumerate()
        .map(|(i, c)| (winner_deck.len() - i) * c)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let p1_deck_init: VecDeque<usize> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .trim()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    let p2_deck_init: VecDeque<usize> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .trim()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    let mut p1_deck = p1_deck_init.clone();
    let mut p2_deck = p2_deck_init.clone();
    let winner_normal = game(&mut p1_deck, &mut p2_deck, false);
    let first = score(winner_normal, &p1_deck, &p2_deck);
    println!("First: {}", first);

    p1_deck = p1_deck_init.clone();
    p2_deck = p2_deck_init.clone();
    let winner_rec = game(&mut p1_deck, &mut p2_deck, true);
    let second = score(winner_rec, &p1_deck, &p2_deck);
    println!("Second: {}", second);
}
