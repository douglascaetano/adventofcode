use std::io;
use std::io::Read;
use std::ops::RangeInclusive;

use regex::Regex;

struct Policy {
    range: RangeInclusive<usize>,
    letter: char,
}

struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn is_valid(&self) -> bool {
        self.policy.range.contains(
            &self
                .password
                .chars()
                .filter(|&ch| ch == self.policy.letter)
                .count(),
        )
    }
}

#[derive(Debug)]
struct EntriesError {
    error: String,
}

impl EntriesError {
    fn new(error: &str) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

impl From<io::Error> for EntriesError {
    fn from(error: io::Error) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

fn main() {
    println!("Advent of Code 2020 - Day 2");

    let entries = get_entries().expect("error getting entries");

    println!("There are {} entries.", entries.len());

    let count_valid = count_valid(&entries);

    println!("From those, {} entries are valid.", count_valid);
}

fn get_entries() -> Result<Vec<Entry>, EntriesError> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let entries = s
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();

            let range_start = (&cap[1]).parse().unwrap();
            let range_end = (&cap[2]).parse().unwrap();
            let letter = (&cap[3]).chars().next().unwrap();
            let password = (&cap[4]).to_string();

            Entry {
                policy: Policy {
                    range: range_start..=range_end,
                    letter,
                },
                password,
            }
        })
        .collect();

    Ok(entries)
}

fn count_valid(entries: &Vec<Entry>) -> usize {
    entries.iter().fold(0, |count, entry| {
        count + (if entry.is_valid() { 1 } else { 0 })
    })
}
