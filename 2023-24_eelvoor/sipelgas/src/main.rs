use std::io::{self, BufRead};

enum Dir {
    Left,
    Right
}
use Dir::*;

impl Dir {
    fn new(s: &str) -> Self {
        match s {
            "V" => Left,
            "P" => Right,
            _ => panic!("Invalid dir")
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let directions: Vec<Dir> = stdin
        .lock()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|c| Dir::new(c))
        .collect();

    println!("{}", result(&directions));
}

fn result(directions: &[Dir]) -> usize {
    0
}