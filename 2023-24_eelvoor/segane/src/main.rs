use std::collections::HashSet;
use std::io::{self, BufRead};

fn all_equal(outputs: &[Vec<char>]) -> bool {
    outputs.windows(2).all(|o| o[0] == o[1])
}

/// Solve it for any with N >= 2, but not too big still (< 11).
/// Recursive function.
///
/// * inq - Remaining chars we need to loop at.
/// * out - Outputs we have build so far.
/// * results - Valid full outputs we found. Using a hash set as we can find multiple times the same.
/// * expected_len - How long the final outputs need to be.
/// * cache - A hash set with outputs, so we don't go looking for things already done.
fn find_output_any_n(
    inq: &[char],
    mut outputs: Vec<Vec<char>>,
    results: &mut HashSet<Vec<char>>,
    expected_len: usize,
    cache: &mut HashSet<Vec<Vec<char>>>,
) {
    // If we have already seen this path.
    // We sort the outputs, as the order in which they are doesn't matter for caching.
    outputs.sort_unstable();
    // We are more likely to have cache hits than misses, so doing a contains without a clone on hits is good.
    if cache.contains(&outputs) {
        return;
    }
    cache.insert(outputs.clone());

    let mut index = 0;
    while index < inq.len() {
        let first = inq[index];
        index += 1;

        // If all outputs are the same, there is no need to explore all of them, we just pick one option.
        if all_equal(&outputs) {
            outputs[0].push(first);
            continue;
        }

        // In which outputs is it possible to put the letter.
        let mut possible = Vec::with_capacity(outputs.len());

        // Find the first free position of each output.
        for (i, out) in outputs.iter().enumerate() {
            let pos = out.len();
            // If a string is full, it cannot be added there.
            if pos == expected_len {
                continue;
            }

            possible.push(i);
            // Check if the letter in any of the other string at this position is the same as the one we are checking.
            for other_out in &outputs {
                if other_out.len() <= pos {
                    continue;
                }
                if other_out[pos] != first {
                    possible.pop();
                    break;
                }
            }
        }

        if possible.is_empty() {
            // No options, giving up.
            break;
        }
        if possible.len() == 1 {
            // Only one option, not going recursive.
            outputs[possible[0]].push(first);
            continue;
        }

        // If we couldn't pick an option, we have to explore all.
        for i in possible {
            let mut new_outputs = outputs.clone();
            new_outputs[i].push(first);
            find_output_any_n(&inq[index..], new_outputs, results, expected_len, cache);
        }

        // If we went to recursively explore sub-paths, we don't look further here.
        break;
    }

    // Once input has been fully explored, results are only valid if they are equals.
    if index == inq.len() && all_equal(&outputs) {
        results.insert(outputs[0].clone());
    }
}

fn find_output(input: &str, count: usize) -> Vec<String> {
    let mut results: HashSet<Vec<char>> = HashSet::default();

    let inq: Vec<char> = input.chars().collect();
    let expected_len = inq.len() / count;

    let outputs: Vec<Vec<char>> = vec![Vec::with_capacity(expected_len); count];

    let mut cache: HashSet<Vec<Vec<char>>> = HashSet::default();

    // eprintln!("Searching {} for {} programs", input, count);
    find_output_any_n(&inq, outputs, &mut results, expected_len, &mut cache);

    let mut res_str: Vec<String> = results.iter().map(|r| r.iter().collect()).collect();
    res_str.sort_unstable();
    // eprintln!("Result: {:?}", res_str);
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
        let result = find_output(&input.1, input.0);
        println!("{}", result.len());
        println!(
            "{}",
            result
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

#[test]
fn test_find_output_n2() {
    assert_eq!(
        find_output(
            "/home//homjukue/ju/lahkeundu/sedl/ahseeganendused/segane",
            2
        ),
        ["/home/juku/lahendused/segane"]
    );
    assert_eq!(
        find_output(
            "/h/ohomme/e/juku/lajukhue/landusehedn/segadunseed/segane",
            2
        ),
        ["/home/juku/lahendused/segane"]
    );
    assert_eq!(find_output("abcdabcd", 2), ["abcd"]);
    assert_eq!(find_output("aabbaabb", 2), ["aabb", "abab"]);
}

#[test]
fn test_extra_n2() {
    assert_eq!(
        find_output("iiiiirirrrirrrirrrrrrr", 2),
        ["iiiirrrrrrr", "iiirirrrrrr", "iiirrirrrrr", "iiirrrirrrr"]
    );
    assert_eq!(
        find_output("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk", 2),
        ["kkkkkkkkkkkkkkkkkkkkkkkk"]
    );
}

#[test]
fn test_find_output_any_n() {
    assert_eq!(find_output("aaaaaaaaaa", 5), ["aa"]);
    assert_eq!(find_output("eerrrroereorrrorrror", 4), ["error"]);
}
