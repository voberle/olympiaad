use std::io;

// Represents a folder with the data provided in the input
#[derive(Debug, PartialEq)]
struct Folder {
    total: u64,
    // positions starts counting at 1. Last one is equal to total
    position: u64,
}

impl Folder {
    fn new(total: u64, position: u64) -> Self {
        Self { total, position }
    }

    fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
        let i = s.find(" ").unwrap();
        let t: u64 = u64::from_str_radix(&s[..i], 10)?;
        let p: u64 = u64::from_str_radix(&s[i + 1..], 10)?;

        Ok(Folder {
            total: t,
            position: p,
        })
    }

    fn reversed(&self) -> u64 {
        self.total - self.position + 1
    }

    fn normal_dist(&self, start: u64) -> u64 {
        u64::abs_diff(start, self.position)
    }

    fn reverse_dist(&self, start: u64) -> u64 {
        start // go to reverse button
            + 1 // go to beginning of list
            + (self.total - self.position) // go from beginning to inversed position
    }
}

#[test]
fn from_str_works() {
    assert_eq!(Folder::from_str("54 129").unwrap(), Folder::new(54, 129));
}

#[test]
fn dist_works() {
    assert_eq!(Folder::new(5, 4).normal_dist(2), 2);
    assert_eq!(Folder::new(6, 5).normal_dist(2), 3);
    assert_eq!(Folder::new(5, 3).normal_dist(1), 2);
    assert_eq!(Folder::new(5, 4).normal_dist(1), 3);
    assert_eq!(Folder::new(5, 4).reverse_dist(1), 3);
}

// Data about a path followed so far
#[derive(Clone)]
struct Path {
    // Index of the file we are targeting
    target: u64,
    // Cost so far
    cost: u64,
}

fn kaustad(folders: Vec<Folder>) -> u64 {
    // At any step in the folder hierarchy, we have two possible paths:
    // One one that goes to target position, and the one that goes to reversed target.
    let start: u64 = 1;
    let mut path_normal = Path {
        target: folders[0].position,
        cost: folders[0].normal_dist(start),
    };
    let mut path_reversed = Path {
        target: folders[0].reversed(),
        cost: folders[0].reverse_dist(start),
    };
    for i in 1..folders.len() {
        // making copies as we will change them
        let old_normal = path_normal.clone();
        let old_reversed = path_reversed.clone();
        path_normal.target = folders[i].position;
        path_normal.cost = u64::min(
            old_normal.cost + folders[i].normal_dist(old_normal.target),
            old_reversed.cost + folders[i].normal_dist(old_reversed.target),
        );
        path_reversed.target = folders[i].reversed();
        path_reversed.cost = u64::min(
            old_normal.cost + folders[i].reverse_dist(old_normal.target),
            old_reversed.cost + folders[i].reverse_dist(old_reversed.target),
        );
    }
    u64::min(path_normal.cost, path_reversed.cost)
}

fn main() {
    let stdin = io::stdin();
    let mut n = String::new();
    stdin.read_line(&mut n).unwrap();
    // We parse immediately for performance reasons
    let mut folders: Vec<Folder> = Vec::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        folders.push(Folder::from_str(&s).unwrap());
    }

    let result = kaustad(folders);
    println!("{}", result);
}

#[test]
fn check() {
    assert_eq!(kaustad(vec![Folder::new(5, 3), Folder::new(2, 1)]), 4);
    assert_eq!(kaustad(vec![Folder::new(5, 4), Folder::new(2, 1)]), 4);
}
