use std::io::{self, Read};
use std::process;
use std::collections::HashMap;
use std::cmp;

fn main() {
    println!("Advent of Code 2018 - Day 2");
    println!("---------------------------");
    println!();

    let mut input = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut input) {
        eprintln!("Input is not a valid UTF-8 string.");
        process::exit(1);
    }

    let ids: Vec<_> = input.lines().collect();

    let ilen = ids.len();
    println!("Your input has {} entr{}.", ilen, if ilen == 1 { "y" } else { "ies" });
    println!();

    let mut two_rep_cnt = 0;
    let mut three_rep_cnt = 0;

    for id in ids.iter() {
        let (two_rep, three_rep) = count_repetition(&id);
        if two_rep { two_rep_cnt += 1; }
        if three_rep { three_rep_cnt += 1; }
    }

    let checksum = two_rep_cnt * three_rep_cnt;

    println!("The checksum for the provided IDs is: {}", checksum);

    'outer: for i in 0..ids.len() {
        for j in i..ids.len() {
            if hamming_distance(&ids[i], &ids[j]) == 1 {
                println!("Found two IDs whose difference is one char:");
                println!("- {}\n- {}", ids[i], ids[j]);
                println!();
                println!("The common letters between the IDs are: {}", common_letters(&ids[i], &ids[j]));
                break 'outer;
            }
        }
    }
}

fn count_repetition(id: &str) -> (bool, bool) {
    let mut letters = HashMap::new();

    for letter in id.chars() {
        let counter = letters.entry(letter).or_insert(0);
        *counter += 1;
    }

    let mut two_rep = false;
    let mut three_rep = false;

    for count in letters.values() {
        match count {
            2 => two_rep = true,
            3 => three_rep = true,
            _ => (),
        }
    }

    (two_rep, three_rep)
}

fn hamming_distance(a: &str, b: &str) -> u32 {
    if a.len() != b.len() {
        panic!("String lengths differ, they must be the same.");
    }

    let mut sum = 0;

    for pair in a.chars().zip(b.chars()) {
        if pair.0 != pair.1 {
            sum += 1;
        }
    }

    sum
}

fn common_letters(a: &str, b: &str) -> String {
    let mut r = String::new();

    let m = cmp::min(a.len(), b.len());
    for i in 0..m {
        // WARNING: UGLY, UGLY CODE BELOW!
        if a[i..].chars().next().unwrap() == b[i..].chars().next().unwrap() {
            r.push(a[i..].chars().next().unwrap());
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_letter_repetition() {
        let ids = vec![
            ("abcdef", (false, false)),
            ("bababc", (true, true)),
            ("abbcde", (true, false)),
            ("abcccd", (false, true)),
            ("aabcdd", (true, false)),
            ("abcdee", (true, false)),
            ("ababab", (false, true))
        ];

        for (id, rep) in ids {
            assert_eq!(rep, count_repetition(&id));
        }
    }

    #[test]
    fn check_hamming_distance() {
        let string_pairs = vec![
            (("abcdef", "abcdef"), 0),
            (("abcdef", "abcddd"), 2),
            (("abcdef", "fedcba"), 6)
        ];

        for (pair, result) in string_pairs {
            assert_eq!(result, hamming_distance(&pair.0, &pair.1));
        }
    }

    #[test]
    #[should_panic]
    fn check_hamming_panic() {
        let _ = hamming_distance("big string", "small string");
    }

    #[test]
    fn check_common_letters() {
        let string_pairs = vec![
            (("abcdef", "abcdef"), "abcdef"),
            (("abcdef", "abceef"), "abcef"),
            (("abcbab", "abcaba"), "abc")
        ];

        for (pair, result) in string_pairs {
            assert_eq!(result, common_letters(&pair.0, &pair.1));
        }
    }
}