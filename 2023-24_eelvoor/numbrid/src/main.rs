use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: HashSet<u32> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut min = u32::MAX;
    let mut max = u32::MIN;
    numbers.iter().for_each(|n| {
        min = min.min(*n);
        max = max.max(*n);
    });

    let unwanted_numbers: Vec<u32> = (min + 1..max).filter(|n| !numbers.contains(n)).collect();

    println!("{}", unwanted_numbers.len());
    println!(
        "{}",
        unwanted_numbers
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
