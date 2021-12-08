use std::fs;
use std::iter;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let mut input_signal: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let iter = 100;

    for _ in 0..iter {
        let mut output_signal: Vec<i32> = vec![];
        for i in 1..input_signal.len() + 1 {
            let pattern = iter::repeat(0)
                .take(i)
                .chain(iter::repeat(1).take(i))
                .chain(iter::repeat(0).take(i))
                .chain(iter::repeat(-1).take(i));
            output_signal.push(
                (input_signal
                    .iter()
                    .zip(pattern.cycle().skip(1))
                    .map(|(s, p)| s * p)
                    .sum::<i32>()
                    % 10)
                    .abs(),
            );
        }
        input_signal = output_signal;
    }

    print!("First: ");
    input_signal.iter().take(8).for_each(|x| print!("{}", x));
    println!();

    let input_signal: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .cycle()
        .take(10000 * input.trim().len())
        .collect();
    let start_index: usize = input[..7].parse().unwrap();

    let mut output_signal = vec![];
    let mut last_column = vec![0; iter + 1];

    for i in (start_index..input_signal.len()).rev() {
        last_column[0] = input_signal[i];
        for j in 1..iter + 1 {
            last_column[j] += last_column[j - 1];
            last_column[j] %= 10;
        }
        output_signal.push(last_column[iter]);
    }
    output_signal.reverse();

    print!("Second: ");
    output_signal.iter().take(8).for_each(|x| print!("{}", x));
    println!();
}
