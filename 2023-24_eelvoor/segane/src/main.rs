use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

/// Trying to solve it with N = 2
/// Recursive function.
///
/// * inq - Remaining chars we need to loop at.
/// * out1 and out2 - What we have build so far.
/// * results - Valid full outputs we found. Using a hash set as we can find multiple times the same.
/// * expected_len - How long the final outputs need to be.
/// * cache - A hash set with (inq, out1, out2), so we don't go looking for things already done.
fn find_output_n2(
    mut inq: VecDeque<char>,
    mut out1: Vec<char>,
    mut out2: Vec<char>,
    results: &mut HashSet<Vec<char>>,
    expected_len: usize,
    cache: &mut HashSet<(VecDeque<char>, Vec<char>, Vec<char>)>,
) {
    // If we have already seen this path.
    if !cache.insert((inq.clone(), out1.clone(), out2.clone())) {
        return;
    }

    while let Some(first) = inq.pop_front() {
        let o1_len = out1.len();
        let o2_len = out2.len();

        // All the cases where we are sure where to put the letter.

        // if we put it to the shortest, does it work? If not we can exclude that option
        if o1_len < o2_len && o2_len < expected_len {
            if out2[o1_len] != first {
                // we cannot put the char into out1, has to go to out2.
                out2.push(first);
                continue;
            }
        } else if o1_len > o2_len && o1_len < expected_len {
            if out1[o2_len] != first {
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

        // If we couldn't pick an option, we have to explore both,
        // but only if we haven't reached the expected length.
        if o1_len < expected_len {
            let mut new_out1 = out1.clone();
            new_out1.push(first);
            find_output_n2(
                inq.clone(),
                new_out1,
                out2.clone(),
                results,
                expected_len,
                cache,
            );
        }

        if o2_len < expected_len {
            let mut new_out2 = out2.clone();
            new_out2.push(first);
            find_output_n2(
                inq.clone(),
                out1.clone(),
                new_out2,
                results,
                expected_len,
                cache,
            );
        }

        // If we went to recursively explore sub-paths, we don't look further here.
        break;
    }

    // Once input is empty, results are only valid if they are equals
    if out1 == out2 && out1.len() == expected_len {
        results.insert(out1);
    }
}

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
/// * cache - A hash set with (inq, outputs), so we don't go looking for things already done.
fn find_output_any_n(
    mut inq: VecDeque<char>,
    mut outputs: Vec<Vec<char>>,
    results: &mut HashSet<Vec<char>>,
    expected_len: usize,
    cache: &mut HashSet<(VecDeque<char>, Vec<Vec<char>>)>,
) {
    // If we have already seen this path.
    let mut outputs_for_cache = outputs.clone();
    // We sort the outputs, as the order in which they are doesn't matter for caching.
    outputs_for_cache.sort_unstable();
    if !cache.insert((inq.clone(), outputs_for_cache.clone())) {
        return;
    }

    while let Some(first) = inq.pop_front() {
        // eprintln!("[{}] outputs={:?}", first, outputs);

        // If all outputs are the same, there is no need to explore all of them, we just pick one option.
        if all_equal(&outputs) {
            outputs[0].push(first);
            // eprintln!("[{}] All equals", first);
            continue;
        }

        // In which outputs is it possible to put the letter.
        let mut possible = vec![true; outputs.len()];

        // Find the first free position of each output.
        for (i, out) in outputs.iter().enumerate() {
            let pos = out.len();
            // If a string is full, it cannot be added there.
            if pos == expected_len {
                possible[i] = false;
                continue;
            }
            // Check if the letter in any of the other string at this position is the same as the one we are checking.
            for other_out in &outputs {
                if other_out.len() <= pos {
                    continue;
                }
                if other_out[pos] != first {
                    possible[i] = false;
                    break;
                }
            }
        }

        let options_count = possible.iter().filter(|v| **v).count();
        if options_count == 0 {
            // No options, giving up.
            break;
        }
        if options_count == 1 {
            // Only one option, not going recursive.
            for (i, v) in possible.iter().enumerate() {
                if *v {
                    outputs[i].push(first);
                    break;
                }
            }
            continue;
        }

        // if inq.is_empty() {
        //     break;
        // }

        // If we couldn't pick an option, we have to explore both,
        // but only if we haven't reached the expected length.
        for (i, v) in possible.iter().enumerate() {
            if *v {
                let mut new_outputs = outputs.clone();
                new_outputs[i].push(first);
                find_output_any_n(inq.clone(), new_outputs, results, expected_len, cache);
            }
        }

        // If we went to recursively explore sub-paths, we don't look further here.
        break;
    }

    // Once input is empty, results are only valid if they are equals
    if inq.is_empty() {
        // eprintln!("Input empty ({}) outputs={:?}", inq.len(), outputs);

        if all_equal(&outputs) {
            results.insert(outputs[0].clone());
        }
    }
}

fn _find_output(input: &str, count: usize) -> Vec<String> {
    assert_eq!(count, 2);

    let mut results: HashSet<Vec<char>> = HashSet::new();

    let out1: Vec<char> = Vec::new();
    let out2: Vec<char> = Vec::new();

    let inq: VecDeque<char> = input.chars().collect();
    let expected_len = inq.len() / 2;

    let mut cache: HashSet<(VecDeque<char>, Vec<char>, Vec<char>)> = HashSet::new();

    // eprintln!("Searching {} for {} programs", input, count);
    find_output_n2(inq, out1, out2, &mut results, expected_len, &mut cache);

    let mut res_str: Vec<String> = results.iter().map(|r| r.iter().collect()).collect();
    res_str.sort_unstable();
    // eprintln!("Result: {:?}", res_str);
    res_str
}

fn find_output(input: &str, count: usize) -> Vec<String> {
    let mut results: HashSet<Vec<char>> = HashSet::default();

    let inq: VecDeque<char> = input.chars().collect();
    let expected_len = inq.len() / count;

    let outputs: Vec<Vec<char>> = vec![Vec::with_capacity(expected_len); count];

    let mut cache: HashSet<(VecDeque<char>, Vec<Vec<char>>)> = HashSet::default();

    // eprintln!("Searching {} for {} programs", input, count);
    find_output_any_n(inq, outputs, &mut results, expected_len, &mut cache);

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
        // if input.0 != 2 {
        //     continue;
        // }
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
