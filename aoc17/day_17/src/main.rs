use std::fs;

fn main() {
    let step: usize = fs::read_to_string("input")
        .expect("Error reading input")
        .trim()
        .parse()
        .unwrap();

    let mut buffer = vec![0];
    let mut pos = 0;
    let mut res = 0;
    for i in 1..50000001 {
        pos = (pos + step) % i + 1;
        if i <= 2017 {
            buffer.insert(pos, i);
        }
        if i == 2017 {
            println!("First: {}", buffer[pos + 1]);
        }
        if pos == 1 {
            res = i;
        }
    }

    println!("Second: {}", res);
}
