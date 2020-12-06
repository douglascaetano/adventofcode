use std::io;
use std::io::Read;

use itertools::Itertools;

fn main() {
    println!("Advent of Code 2020 - Day 1");

    let entries = get_entries();

    println!("Total entries: {}", entries.len());

    let pair = find_pair(entries);

    println!("Found pair:");
    println!("  {} + {} = {}", pair[0], pair[1], pair[0] + pair[1]);
    println!("  {} * {} = {}", pair[0], pair[1], pair[0] * pair[1]);
}

fn get_entries() -> Vec<u32> {
    let mut s = String::new();
    io::stdin()
        .read_to_string(&mut s)
        .expect("only valid UTF-8 input");
    s.lines()
        .map(|s| s.parse().expect("only valid integers"))
        .collect()
}

fn find_pair(entries: Vec<u32>) -> Vec<u32> {
    entries
        .into_iter()
        .combinations(2)
        .find(|v| v[0] + v[1] == 2020)
        .expect("input containing a pair of values that sum 2020")
}
