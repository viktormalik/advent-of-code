use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading the file");

    let start: u32 = input.lines().nth(0).unwrap().parse().unwrap();
    let buses: Vec<Option<u32>> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| match n.parse() {
            Ok(x) => Some(x),
            _ => None,
        })
        .collect();

    let min_bus = buses
        .iter()
        .filter(|b| b.is_some())
        .map(|b| ((start / b.unwrap() + 1) * b.unwrap(), b.unwrap()))
        .min_by_key(|&(t, _)| t)
        .unwrap();
    println!("First: {}", (min_bus.0 - start) * min_bus.1);

    let max1 = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .max_by_key(|(_, b)| b.unwrap())
        .unwrap();
    let max2 = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some() && b.unwrap() != max1.1.unwrap())
        .max_by_key(|(_, b)| b.unwrap())
        .unwrap();

    let nums1: Vec<u64> = buses
        .iter()
        .enumerate()
        .filter(|&(i, b)| b.is_some() && (i as i32 - max1.0 as i32).abs() % b.unwrap() as i32 == 0)
        .map(|(_, b)| b.unwrap() as u64)
        .collect();
    let nums2: Vec<u64> = buses
        .iter()
        .enumerate()
        .filter(|&(i, b)| b.is_some() && (i as i32 - max2.0 as i32).abs() % b.unwrap() as i32 == 0)
        .map(|(_, b)| b.unwrap() as u64)
        .collect();

    let lcm1 = nums1.iter().fold(1, |acc, x| acc * x);
    let lcm2 = nums2.iter().fold(1, |acc, x| acc * x);

    let mut x: u64 = lcm1;
    loop {
        if (x + max2.0 as u64 - max1.0 as u64) % lcm2 == 0 {
            println!("Second: {}", x - max1.0 as u64);
            break;
        }
        x += lcm1;
    }
}
