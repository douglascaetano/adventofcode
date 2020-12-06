use std::io;
use std::io::Read;

use itertools::Itertools;
use structopt::StructOpt;

/// Reads entries from stdin and find a combination of given size that sums to the given value.
#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2001")]
struct Opt {
    /// Size of the desired combination of entries to find the sum
    #[structopt(short, long, default_value = "2")]
    combination_size: usize,

    /// Sum to be found in combinations
    #[structopt(short, long, default_value = "2020")]
    sum: u32,
}

fn main() {
    let opt = Opt::from_args();

    println!("Advent of Code 2020 - Day 1");

    let entries = get_entries();

    println!("Total entries: {}", entries.len());

    let comb = match find_combination(opt.combination_size, opt.sum, entries) {
        Some(comb) => comb,
        None => {
            println!("No combination of entries found that match the given criteria.");
            std::process::exit(1);
        }
    };

    println!("Found combination: {:?}", comb);
    println!("      Sum: {}", comb.iter().sum::<u32>());
    println!("  Product: {}", comb.iter().product::<u32>());
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

fn find_combination(size: usize, sum: u32, entries: Vec<u32>) -> Option<Vec<u32>> {
    entries
        .into_iter()
        .combinations(size)
        .find(|v| sum == v.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pair() {
        let pair = find_combination(vec![2, 10, 1000, 9999, 1020]).expect("a pair");
        assert_eq!(pair, vec![1000, 1020]);
        assert_eq!(pair.len(), 2);
    }

    #[test]
    fn test_find_pair_fail() {
        let pair = find_combination(vec![2, 10, 1000, 9999]);
        assert!(pair.is_none());
    }
}
