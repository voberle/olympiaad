use std::io::{self, Read};

fn main() {
    let roads = vec![
        vec![1, 6, 2],
        vec![1, 6, 2],
        vec![1, 8, 2],
        vec![1, 8, 5],
        vec![3, 6, 2],
        vec![3, 6, 2],
        vec![3, 8, 2],
        vec![3, 8, 5],
    ];
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let r: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    
    if roads.contains(&r) {
        println!("JAH");
    } else {
        println!("EI");
    }
}
