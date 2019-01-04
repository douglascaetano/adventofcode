use std::io::{self, Read};
use std::process;
use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2018 - Day 2");
    println!("---------------------------");
    println!();

    let mut input = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut input) {
        eprintln!("Input is not a valid UTF-8 string.");
        process::exit(1);
    }

    let ilen = input.len();
    println!("Your input has {} entr{}.", ilen, if ilen == 1 { "y" } else { "ies" });
    println!();

    let mut two_rep_cnt = 0;
    let mut three_rep_cnt = 0;

    for id in input.lines() {
        let (two_rep, three_rep) = count_repetition(&id);
        if two_rep { two_rep_cnt += 1; }
        if three_rep { three_rep_cnt += 1; }
    }

    let checksum = two_rep_cnt * three_rep_cnt;

    println!("The checksum for the provided IDs is: {}", checksum);
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
}