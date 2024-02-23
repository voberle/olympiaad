use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut number: Vec<u32> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .map(|v| v.parse().unwrap())
        .collect();

    number.sort_unstable();

    let unwanted_numbers: Vec<u32> = (number[0] + 1..*number.last().unwrap())
        .into_iter()
        .filter(|n| !number.contains(n))
        .collect();

    println!("{}", unwanted_numbers.len());
    println!(
        "{}",
        unwanted_numbers
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    // for n in unwanted_numbers {
    //     print!("{}", n);
    //     if
    // }
}
