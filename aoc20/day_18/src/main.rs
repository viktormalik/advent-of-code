use std::fs;

fn eval(expr: &Vec<char>, i: &mut usize, has_prec: bool, plus_prec: bool) -> u64 {
    let mut result = 0;
    while *i < expr.len() {
        if expr[*i].is_whitespace() {
            *i += 1;
        } else if expr[*i].is_digit(10) {
            result = expr[*i].to_digit(10).unwrap() as u64;
            *i += 1;
        } else if expr[*i] == '*' {
            if has_prec {
                return result;
            }
            *i += 1;
            let right = eval(expr, i, !plus_prec, plus_prec);
            result *= right;
        } else if expr[*i] == '+' {
            if has_prec {
                return result;
            }
            *i += 1;
            let right = eval(expr, i, true, plus_prec);
            result += right;
        } else if expr[*i] == '(' {
            *i += 1;
            result = eval(expr, i, false, plus_prec);
            *i += 1;
        } else if expr[*i] == ')' {
            return result;
        }
    }
    return result;
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let first: Vec<u64> = input
        .lines()
        .map(|line| eval(&line.chars().collect(), &mut 0, false, false))
        .collect();
    println!("First: {}", first.iter().sum::<u64>());

    let second: Vec<u64> = input
        .lines()
        .map(|line| eval(&line.chars().collect(), &mut 0, false, true))
        .collect();

    println!("Second: {}", second.iter().sum::<u64>());
}
