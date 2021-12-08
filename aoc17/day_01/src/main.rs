use std::fs;

struct Captcha(Vec<char>);

struct SuccPairsIter<'a> {
    inner: &'a Captcha,
    pos: usize,
}

impl<'a> Iterator for SuccPairsIter<'a> {
    type Item = (&'a char, &'a char);

    fn next(&mut self) -> Option<Self::Item> {
        let l = self.inner.0.len();
        if self.pos >= l {
            None
        } else {
            self.pos += 1;
            Some((
                self.inner.0.get(self.pos - 1).unwrap(),
                self.inner.0.get(self.pos % l).unwrap(),
            ))
        }
    }
}

struct HalfwayPairsIter<'a> {
    inner: &'a Captcha,
    pos: usize,
}

impl<'a> Iterator for HalfwayPairsIter<'a> {
    type Item = (&'a char, &'a char);

    fn next(&mut self) -> Option<Self::Item> {
        let l = self.inner.0.len();
        if self.pos >= l {
            None
        } else {
            self.pos += 1;
            Some((
                self.inner.0.get(self.pos - 1).unwrap(),
                self.inner.0.get((self.pos - 1 + l / 2) % l).unwrap(),
            ))
        }
    }
}

impl Captcha {
    fn succ_pairs<'a>(&'a self) -> SuccPairsIter<'a> {
        SuccPairsIter {
            inner: self,
            pos: 0,
        }
    }

    fn halfway_pairs<'a>(&'a self) -> HalfwayPairsIter<'a> {
        HalfwayPairsIter {
            inner: self,
            pos: 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let captcha = Captcha(input.trim().chars().collect());
    let first: u32 = captcha
        .succ_pairs()
        .filter(|p| p.0 == p.1)
        .map(|p| p.0.to_digit(10).unwrap())
        .sum();
    println!("First:  {}", first);

    let second: u32 = captcha
        .halfway_pairs()
        .filter(|p| p.0 == p.1)
        .map(|p| p.0.to_digit(10).unwrap())
        .sum();
    println!("Second: {}", second);
}
