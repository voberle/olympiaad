use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<u64> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(' ')
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
