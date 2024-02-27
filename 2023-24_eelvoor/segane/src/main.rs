use std::io::{self, BufRead};
use std::collections::{HashSet, VecDeque};

const DEBUG: bool = false;

// Trying to solve it with N = 2
fn find_output_n2(mut inq: VecDeque<char>, mut out1: Vec<char>, mut out2: Vec<char>, results: &mut HashSet<Vec<char>>, expected_len: usize) {
    if DEBUG {
        // println!("inq={:?}: {:?} {:?}", inq, out1, out2);
        // println!("inq.len={}: o1={} o2={}, exp={}", inq.len(), out1.len(), out2.len(), expected_len);
    }

    // If the remaining letters don't contain what we need for the shortest to catchup on the longest
    // there is no need to check further
    if out1.len() < out2.len() {
        if !inq.contains(&out2[out1.len()]) {
            return;
        }
    } else if out2.len() < out1.len() {
        if !inq.contains(&out1[out2.len()]) {
            return;
        }
    }


    while let Some(first) = inq.pop_front() {
        // The shortest part has to be equal to the beginning of the other part.
        // That check is probably not needed.
        if out1.len() < out2.len() {
            if out1 != out2[0..out1.len()] {
                return;
            }
        } else {
            if out1[0..out2.len()] != out2 {
                return;
            }
        }

        // println!("{first}: {:?} {:?}", out1, out2);
        let o1_len = out1.len();
        let o2_len = out2.len();
        // if we put it to the shortest, does it work? If not we can exclude that option
        if o1_len < o2_len && o2_len < expected_len {
            let pos = o1_len;
            if out2[pos] != first {
                // we cannot put the char into out1, has to go to out2.
                out2.push(first);
                continue;
            }
        } else if o1_len > o2_len && o1_len < expected_len {
            let pos = o2_len;
            if out1[pos] != first {
                out1.push(first);
                continue;
            }
        } else if o1_len == o2_len && o1_len < expected_len {
            // If the strings are the same, we just pick one option
            if out1 == out2 {
                out1.push(first);
                continue;
            }
        } else if o1_len == expected_len {
            // If one string is full, we pick the other one
            out2.push(first);
            continue;        
        } else if o2_len == expected_len {
            out1.push(first);
            continue;        
        }

        if inq.is_empty() {
            break;
        }

        // any other case, we have two options, can go to either
        if o1_len < expected_len {
            let mut new_out1 = out1.clone();
            new_out1.push(first);
            find_output_n2(inq.clone(), new_out1, out2.clone(), results, expected_len);
        }

        if o2_len < expected_len {
            let mut new_out2 = out2.clone();
            new_out2.push(first);
            find_output_n2(inq.clone(), out1.clone(), new_out2, results, expected_len);
        }

        break;
    }

    // once input is empty, results are only valid if they are equals
    if out1 == out2 && out1.len() == expected_len {
        if DEBUG {
            // println!("Insert {:?}", out1);
        }
        results.insert(out1);
    }

}

fn find_output(input: &str, count: usize) -> Vec<String> {
    assert_eq!(count, 2);

    let mut results: HashSet<Vec<char>> = HashSet::new();

    let out1: Vec<char> = Vec::new();
    let out2: Vec<char> = Vec::new();

    let inq: VecDeque<char> = input.chars().collect();
    let expected_len = inq.len() / 2;

    if DEBUG {
        println!("Searching {} for {} programs", input, count);
    }
    find_output_n2(inq, out1, out2, &mut results, expected_len);

    let mut res_str: Vec<String> = results.iter().map(|r| r.iter().collect()).collect();
    res_str.sort_unstable();
    if DEBUG {
        println!("Result: {:?}", res_str);
    }
    res_str
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines().skip(1);
    let mut inputs: Vec<(usize, String)> = Vec::new();
    while let Some(Ok(l)) = it.next() {
        let count: usize = l.parse().unwrap();
        let input = it.next().unwrap().unwrap();
        inputs.push((count, input));
    }

    for input in inputs {
        if input.0 != 2 {
            continue;
        }
        let result = find_output(&input.1, input.0);
        println!("{}", result.len());
        println!("{}", result.iter().map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" "));
    }
}

#[test]
fn test_find_output_n2() {
    assert_eq!(find_output("/home//homjukue/ju/lahkeundu/sedl/ahseeganendused/segane", 2), ["/home/juku/lahendused/segane"]);
    assert_eq!(find_output("/h/ohomme/e/juku/lajukhue/landusehedn/segadunseed/segane", 2), ["/home/juku/lahendused/segane"]);
    assert_eq!(find_output("abcdabcd", 2), ["abcd"]);
    assert_eq!(find_output("aabbaabb", 2), ["aabb", "abab"]);
}

#[test]
fn test_other_1() {
    assert_eq!(find_output("iiiiirirrrirrrirrrrrrr", 2), ["iiiirrrrrrr", "iiirirrrrrr", "iiirrirrrrr", "iiirrrirrrr"]);
}

#[test]
fn test_other_2() {
    assert_eq!(find_output("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk", 2), ["kkkkkkkkkkkkkkkkkkkkkkkk", "kkkkkkkkkkkkkkkkkkkkkkkk"]);
}

    // assert_eq!(find_output("aaaaaaaaaa", 5), ["aa"]);
    // assert_eq!(find_output("eerrrroereorrrorrror", 4), ["error"]);
