use modinverse::modinverse;
use std::fs;

fn modulo(n: i64, base: i64) -> i64 {
    let res = n % base;
    match res >= 0 {
        true => res,
        false => res + base,
    }
}

fn modulo_mul(x: i64, y: i64, base: i64) -> i64 {
    let mul = x as i128 * y as i128;
    let res = mul % base as i128;
    modulo(res as i64, base)
}

fn composition(f: (i64, i64), g: (i64, i64), base: i64) -> (i64, i64) {
    (
        modulo_mul(f.0, g.0, base),
        modulo(modulo_mul(f.1, g.0, base) + g.1, base),
    )
}

fn apply(f: (i64, i64), x: i64, base: i64) -> i64 {
    modulo(modulo_mul(f.0, x, base) + f.1, base)
}

fn repeat_function(fun: (i64, i64), iterations: i64, base: i64) -> (i64, i64) {
    let mut bit_fun = fun;
    let mut bit = 1;

    let mut res_fun = (1, 0);

    while bit <= iterations {
        if iterations & bit != 0 {
            res_fun = composition(res_fun, bit_fun, base);
        }
        bit_fun = composition(bit_fun, bit_fun, base);
        bit *= 2;
    }
    res_fun
}

fn get_fun(technique: &str, cards: i64, forward: bool) -> (i64, i64) {
    if technique == "deal into new stack" {
        return (-1, -1);
    } else {
        let num: i64 = technique.split(" ").last().unwrap().parse().unwrap();

        if technique.starts_with("cut") {
            return (
                1,
                match forward {
                    true => -num,
                    false => num,
                },
            );
        } else {
            return (
                match forward {
                    true => num,
                    false => modinverse(num, cards).unwrap(),
                },
                0,
            );
        }
    }
}

fn forward_function(moves: &String, cards: i64) -> (i64, i64) {
    moves.lines().fold((1, 0), |fun, m| {
        composition(fun, get_fun(m, cards, true), cards)
    })
}

fn backward_function(moves: &String, cards: i64) -> (i64, i64) {
    moves.lines().rev().fold((1, 0), |fun, m| {
        composition(fun, get_fun(m, cards, false), cards)
    })
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let fun = forward_function(&input, 10007);
    let first = apply(fun, 2019, 10007);
    println!("First: {}", first);

    let fun = backward_function(&input, 119315717514047);
    let repeat_fun = repeat_function(fun, 101741582076661, 119315717514047);
    let second = apply(repeat_fun, 2020, 119315717514047);
    println!("Second: {}", second);
}
