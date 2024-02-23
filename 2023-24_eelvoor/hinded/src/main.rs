use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();
    let len: usize = it.next().unwrap().unwrap().parse::<usize>().unwrap();
    // println!("{}", len);

    let grades: Vec<i32> = it
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let money_change: Vec<i32> = grades.iter().map(|v| v - 50).collect();
    // print_grid(&money_change, len);

    // we have a square
    let pos = |row, col| row * len + col;

    // https://en.wikipedia.org/wiki/Summed-area_table
    let mut summed_area = vec![0; len * len];

    // First lines
    for c in 0..len {
        summed_area[pos(0, c)] = money_change[pos(0, c)];
    }
    for r in 0..len {
        summed_area[pos(r, 0)] = money_change[pos(r, 0)];
    }
    // Remaining lines
    for r in 1..len {
        for c in 1..len {
            summed_area[pos(r, c)] =
                money_change[pos(r, c)] + summed_area[pos(r - 1, c)] + summed_area[pos(r, c - 1)]
                    - summed_area[pos(r - 1, c - 1)];
        }
    }
    // print_grid(&summed_area, len);

    let sum: i32 = money_change.iter().sum();
    // println!("{}", sum);

    // let mut max: i32 = 0;
    let mut min: i32 = i32::MAX;
    for r in 0..len {
        for c in 0..len {
            let a = (r, c);
            let s = money_change[pos(a.0, a.1)];
            min = min.min(s);
            for tr in r + 1..len {
                for tc in c + 1..len {
                    let b = (r, tc);
                    let c = (tr, c);
                    let d = (tr, tc);
                    let s = summed_area[pos(d.0, d.1)] + summed_area[pos(a.0, a.1)]
                        - summed_area[pos(b.0, b.1)]
                        - summed_area[pos(c.0, c.1)];
                    min = min.min(s);
                    // println!("{:?} {:?} {:?} {:?} - {}", a, b, c, d, min);
                }
            }
        }
    }

    println!("{}", sum - min);
}

fn print_grid(grid: &[i32], len: usize) {
    for row in 0..len {
        for p in row * len..(row + 1) * len {
            let c = grid[p];
            print!("{} ", c);
        }
        println!();
    }
}
