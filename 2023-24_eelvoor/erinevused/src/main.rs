use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let numbers: Vec<u64> = stdin
        .lock()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let sum: u64 = numbers
        .iter()
        .enumerate()
        .map(|(i, a)| {
            numbers
                .iter()
                .skip(i + 1)
                .map(|b| a.abs_diff(*b))
                .sum::<u64>()
        })
        .sum();
    println!("{}", sum);
}
