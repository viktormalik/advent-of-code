use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let vec: Vec<_> = input.trim().chars().collect();

    let mut copy = vec.to_vec();
    reduce(&mut copy);
    println!("First: {}", copy.len());

    let second = { 65u8..91 }
        .map(|i| {
            let mut copy = vec.to_vec();
            remove(&mut copy, i as char);
            reduce(&mut copy);
            copy.len()
        })
        .min()
        .unwrap();

    println!("Second: {}", second);
}

fn remove(vec: &mut Vec<char>, c: char) {
    vec.retain(|&x| x.to_ascii_uppercase() != c);
}

fn reduce(vec: &mut Vec<char>) {
    let mut i = 0;
    while i < (vec.len() - 1) {
        if (vec[i] as i32 - vec[i + 1] as i32).abs() == 32 {
            vec.remove(i);
            vec.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
}
