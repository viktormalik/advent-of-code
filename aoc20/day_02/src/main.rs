use std::fs;

struct Pass {
    from: usize,
    to: usize,
    c: char,
    pass: String,
}

impl Pass {
    fn valid_old(&self) -> bool {
        let cnt = self.pass.chars().filter(|&c| c == self.c).count();
        cnt >= self.from && cnt <= self.to
    }

    fn valid_new(&self) -> bool {
        (self.pass.as_bytes()[self.from - 1] == self.c as u8)
            ^ (self.pass.as_bytes()[self.to - 1] == self.c as u8)
    }
}

fn parse(line: &str) -> Pass {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let nums: Vec<&str> = parts[0].split('-').collect();
    Pass {
        from: nums[0].parse().unwrap(),
        to: nums[1].parse().unwrap(),
        c: parts[1].chars().nth(0).unwrap(),
        pass: parts[2].to_string(),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");
    let passwords: Vec<Pass> = input.lines().map(|l| parse(l)).collect();

    let first = passwords.iter().filter(|p| p.valid_old()).count();
    println!("First: {}", first);

    let second = passwords.iter().filter(|p| p.valid_new()).count();
    println!("Second: {}", second);
}
