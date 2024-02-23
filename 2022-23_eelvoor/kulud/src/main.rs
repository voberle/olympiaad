use std::io;

#[derive(Debug)]
struct Wire {
    from: usize,
    to: usize,
    sum: u32,
}

impl Wire {
    fn new(from: usize, to: usize, sum: u32) -> Self {
        Self { from, to, sum }
    }

    fn to_string(&self) -> String {
        // Output wants to start counting friends at 1, not 0
        format!("{} {} {}", self.from + 1, self.to + 1, self.sum)
    }
}

// 4 2      16
// 1 12     3
// 2 20     4 1 4
//          4 2 4
//          3 2 8
// Warning: Input data starts to count friends at 1, not 0.
fn wires(
    n_m_str: String,
    expenses_str: Vec<String>,
) -> (u32, usize, Vec<String>) {
    let n_m: Vec<usize> = n_m_str.trim().split(" ").map(|i| i.parse().unwrap()).collect();
    let (n, m) = (n_m[0], n_m[1]);
    let mut expenses: Vec<(usize, u32)> = Vec::new();
    for e in expenses_str {
        let xs: Vec<u32> = e.trim().split(" ").map(|i| i.parse().unwrap()).collect();
        expenses.push((
            (xs[0] as usize) - 1, // Start counting friends a 0
            xs[1]
        ));
    }

    let mut wires: Vec<Wire> = Vec::new();

    // Find total paid
    let total_paid: u32 = expenses.iter().map(|i| i.1).sum();
    // Each friend's share is total / number of friends
    let per_friend: u32 = total_paid / n as u32;

    // We want to have the the total amount transferred as small as possible (and not the total number of transfers).
    // we classify the friends by how much they paid,
    // and we wire from the one who paid least to the one who paid most
    let mut paid_per_friend:  Vec<(usize, u32)> = Vec::new();
    for i in 0..n {
        paid_per_friend.push(
            (i, expenses.iter()
                .filter(|e| e.0 == i)
                .map(|f| f.1)
                .sum())
        );
    }

    paid_per_friend.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    //dbg!(&paid_per_friend);

    let mut j = paid_per_friend.len() - 1;
    for i in 0..n {
        while paid_per_friend[i].1 < per_friend {
            // Find receiver
            while paid_per_friend[j].1 <= per_friend {
                j -= 1;
            }
            let to_pay = u32::min(
                per_friend - paid_per_friend[i].1, 
                paid_per_friend[j].1 - per_friend
            );
            wires.push(Wire::new(paid_per_friend[i].0, paid_per_friend[j].0, to_pay));
            paid_per_friend[i].1 += to_pay;
            paid_per_friend[j].1 -= to_pay;
        }
    }

    (
        wires.iter().map(|f| f.sum).sum(), 
        wires.len(), 
        wires.iter().map(|f| f.to_string()).collect()
    )
}

// $ export N=5
// $ cat resources/test/input/input$N.txt | cargo r > out; head out; head resources/test/output/output$N.txt
fn main() {
    let stdin = io::stdin();
    let mut n_m_str = String::new();
    stdin.read_line(&mut n_m_str).unwrap();

    let mut expenses_str: Vec<String> = Vec::new();
    for line in stdin.lines() {
        expenses_str.push(line.unwrap());
    }

    let result = wires(n_m_str, expenses_str);
    
    println!("{}", result.0);
    println!("{}", result.1);
    for l in result.2 {
        println!("{}", l);
    }
}

#[test]
fn check() {
    assert_eq!(
        wires("4 2".into(), vec!["1 12".into(), "2 20".into()]),
        (16, 3, vec!["4 1 4".into(), "4 2 4".into(), "3 2 8".into()])
    );
}