use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let numbers: Vec<usize> = stdin
        .lock()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    // println!("{}", brute_force(&numbers));
    println!("{}", optimized(&numbers));
}

fn brute_force(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .enumerate()
        .map(|(i, a)| {
            numbers
                .iter()
                .skip(i + 1)
                .map(|b| a.abs_diff(*b))
                .sum::<usize>()
        })
        .sum()
}

fn optimized(numbers: &[usize]) -> usize {
    // We find how many pairs there are of each combination.
    // For that, we just need to know how many occurences there are of each number.
    // So for number A and B, there are occ(A) * occ(B) pairs.
    let max = *numbers.iter().max().unwrap();
    let mut occurences = vec![0; max + 1];
    for n in numbers {
        occurences[*n] += 1;
    }

    let mut sum = 0;
    for a in 0..=max {
        for b in a..=max {
            sum += (b - a) * occurences[a] * occurences[b];
        }
    }
    sum
}
