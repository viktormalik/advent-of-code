use std::fs;

type Pattern = Vec<Vec<char>>;
type Rule = (Pattern, Pattern);

fn parse_rule(line: &str) -> Rule {
    let parts: Vec<&str> = line.split("=>").map(|p| p.trim()).collect();
    (parse_pattern(parts[0]), parse_pattern(parts[1]))
}

fn parse_pattern(p: &str) -> Pattern {
    p.split('/').map(|r| r.chars().collect()).collect()
}

fn rotate(p: &Pattern) -> Pattern {
    (0..p.len())
        .map(|i| p.iter().map(|row| row[i]).rev().collect())
        .collect()
}

fn flip(p: &Pattern) -> Pattern {
    p.iter()
        .map(|row| row.iter().rev().map(|&c| c).collect())
        .collect()
}

fn matches(pattern: &Pattern, image: &Pattern) -> bool {
    let mut p = pattern.clone();
    for _ in 0..4 {
        p = rotate(&p);
        if p == *image {
            return true;
        }
    }
    p = flip(&p);
    for _ in 0..4 {
        p = rotate(&p);
        if p == *image {
            return true;
        }
    }
    false
}

fn find_match<'a>(p: &'a Pattern, rules: &'a Vec<Rule>) -> &'a Pattern {
    &rules.iter().find(|r| matches(&r.0, p)).unwrap().1
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let rules: Vec<Rule> = input.lines().map(|l| parse_rule(l)).collect();

    let mut image: Vec<Vec<char>> = [".#.", "..#", "###"]
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    for i in 0..18 {
        let step = match image.len() % 2 == 0 {
            true => 2,
            false => 3,
        };
        let new_step = step + 1;
        let new_size = image.len() * new_step / step;

        let mut new_image = vec![vec!['.'; new_size]; new_size];

        for start_r in (0..image.len()).step_by(step) {
            for start_c in (0..image.len()).step_by(step) {
                let square: Pattern = (0..step)
                    .map(|r| (0..step).map(|c| image[start_r + r][start_c + c]).collect())
                    .collect();
                let new_square = find_match(&square, &rules);
                for r in 0..new_step {
                    for c in 0..new_step {
                        new_image[start_r * new_step / step + r][start_c * new_step / step + c] =
                            new_square[r][c];
                    }
                }
            }
        }

        image = new_image;

        if i == 4 {
            let first: usize = image
                .iter()
                .map(|r| r.iter().filter(|&x| *x == '#').count())
                .sum();
            println!("First: {}", first);
        }
    }

    let second: usize = image
        .iter()
        .map(|r| r.iter().filter(|&x| *x == '#').count())
        .sum();
    println!("Second: {}", second);
}
