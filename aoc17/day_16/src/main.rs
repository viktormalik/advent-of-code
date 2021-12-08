use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut programs: Vec<char> = ('a'..'q').collect();

    let mut seen = HashMap::new();
    let mut i = 0;
    while i < 1000000000 {
        for c in input.trim().split(',') {
            let cmd = c.as_bytes()[0] as char;
            if cmd == 's' {
                let n: usize = c[1..].parse().unwrap();
                programs = programs
                    .iter()
                    .skip(programs.len() - n)
                    .chain(programs.iter().take(programs.len() - n))
                    .map(|&n| n)
                    .collect();
            } else if cmd == 'x' {
                let pos1: usize = c[1..].split('/').nth(0).unwrap().parse().unwrap();
                let pos2: usize = c[1..].split('/').nth(1).unwrap().parse().unwrap();
                programs.swap(pos1, pos2);
            } else if cmd == 'p' {
                let p1 = c.as_bytes()[1] as char;
                let p2 = c.as_bytes()[3] as char;
                let pos1 = programs.iter().position(|&p| p == p1).unwrap();
                let pos2 = programs.iter().position(|&p| p == p2).unwrap();
                programs.swap(pos1, pos2);
            }
        }

        let string = programs.iter().collect::<String>();

        if i == 0 {
            println!("First: {}", string);
        }

        let s = seen.get(&string);
        if s.is_some() {
            i = 1000000000 - (1000000000 - i) % (i - s.unwrap());
        }
        seen.insert(string, i);
        i += 1;
    }

    println!("Second: {}", programs.iter().collect::<String>());
}
