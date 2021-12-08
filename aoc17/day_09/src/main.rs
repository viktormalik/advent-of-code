use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut level = 0;
    let mut score = 0;
    let mut garbage_size = 0;
    let mut garbage = false;
    let mut skip = false;
    for c in input.trim().chars() {
        if skip {
            skip = false;
            continue;
        }

        if c == '{' && !garbage {
            level += 1;
        } else if c == '}' && !garbage {
            score += level;
            level -= 1;
        } else if c == '<' && !garbage {
            garbage = true;
        } else if c == '>' {
            garbage = false;
        } else if c == '!' {
            skip = true;
        } else if garbage {
            garbage_size += 1;
        }
    }

    println!("First: {}", score);
    println!("Second: {}", garbage_size);
}
