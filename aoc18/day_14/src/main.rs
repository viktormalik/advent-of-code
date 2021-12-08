use std::fs;

fn main() {
    let in_str = fs::read_to_string("input").expect("Error reading input");
    let input_vec: Vec<usize> = in_str
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let input: usize = in_str.trim().parse().unwrap();
    let input_len = in_str.trim().len();

    let mut recipes = vec![3, 7];
    let mut i1 = 0;
    let mut i2 = 1;

    let mut first = false;
    let mut second = false;
    loop {
        let sum = recipes[i1] + recipes[i2];
        if sum >= 10 {
            recipes.push(sum / 10);
        }
        recipes.push(sum % 10);
        i1 = (i1 + recipes[i1] + 1) % recipes.len();
        i2 = (i2 + recipes[i2] + 1) % recipes.len();

        if !first && recipes.len() >= input + 10 {
            print!("First: ");
            for r in recipes.iter().skip(input).take(10) {
                print!("{}", r);
            }
            println!();
            if second {
                break;
            }
            first = true;
        }

        if !second && input_len < recipes.len() {
            if &recipes[recipes.len() - input_len..recipes.len()] == input_vec.as_slice() {
                println!("Second: {}", recipes.len() - input_len);
                if first {
                    break;
                }
                second = true;
            }
            if &recipes[recipes.len() - input_len - 1..recipes.len() - 1] == input_vec.as_slice() {
                println!("Second: {}", recipes.len() - input_len - 1);
                if first {
                    break;
                }
                second = true;
            }
        }
    }
}
