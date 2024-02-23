use std::io;

// 100 101 103 104 104 103 103 102
//     +1  +2  0   0   +1  -1  -1
//     M   M   T   T   V   M    M
fn soit(
    dist_str: String,
    height_str: String,
    change_str: String
) -> String {
    // good to clean input with trim() to remove trailing new line
    let dist: usize = dist_str.trim().parse().unwrap();
    let height: Vec<i32> = height_str.trim().split(" ").map(|i| i.parse().unwrap()).collect();
    let change: Vec<i32> = change_str.trim().split(" ").map(|i| i.parse().unwrap()).collect();
    let mut result: Vec<String> = Vec::new();
    let mut real_height = height[0];
    for i in 0..dist {
        real_height = real_height + change[i];
        let diff = height[i + 1] - real_height;
        result.push(
            if diff == 0 {
                "M"
            } else if diff > 0 {
                "T"
            } else {
                "V"
            }.to_string()
        );
    }
    result.join("")
}

fn main() {
    let mut dist = String::new();
    io::stdin().read_line(&mut dist).unwrap();
    let mut height = String::new();
    io::stdin().read_line(&mut height).unwrap();
    let mut change = String::new();
    io::stdin().read_line(&mut change).unwrap();
    let result = soit(dist, height, change);
    println!("{}", result);
}

#[test]
fn check() {
    assert_eq!(soit("7".to_string(), "100 101 103 104 104 103 103 102".to_string(), "+1 +2 0 0 +1 -1 -1".to_string()), "MMTTVMM");
}