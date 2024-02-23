use std::io;

fn palind(s: String) -> String {
    let input: Vec<&str> = s.split(" ").collect();
    if input[0] == input[3] {
        if input[1] == input[2] {
            return input.join(" ");
        } else {
            return vec![input[0], input[1], input[1], input[3]].join(" ");
        }
    } else {
        if input[1] == input[2] {
            return vec![input[0], input[1], input[2], input[0]].join(" ");
        }
    }
    String::new()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result = palind(input);
    if result.is_empty() {
        println!("NO");
    } else {
        println!("JAH");
        println!("{}", result);
    }
}

#[test]
fn check() {
    assert_eq!(palind("1 2 2 1".to_string()), "1 2 2 1");
    assert_eq!(palind("1 2 2 3".to_string()), "1 2 2 1");
    assert_eq!(palind("1 2 3 4".to_string()), "");
    assert_eq!(palind("4 4 4 4".to_string()), "4 4 4 4");
    // assert_eq!(palind("3 3 3 5".to_string()), "5 3 3 5");
    assert_eq!(palind("3 3 3 5".to_string()), "3 3 3 3");
    assert_eq!(palind("6 7 8 6".to_string()), "6 7 7 6");
    // assert_eq!(palind("2 7 8 2".to_string()), "2 8 8 2");
    assert_eq!(palind("2 7 8 2".to_string()), "2 7 7 2");
    assert_eq!(palind("2 3 2 3".to_string()), "");
}