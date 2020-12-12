mod entry;
mod policies;

use std::io;
use std::io::Read;

use structopt::StructOpt;

use crate::entry::Entries;
use crate::policies::PolicyType;

/// Reads entries from stdin and checks if passwords comply to the chosen policy.
#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2002")]
struct Opt {
    /// Policy to enforce
    #[structopt(short, long, default_value = "old")]
    policy: PolicyType,
}

fn main() {
    let opt = Opt::from_args();

    println!("Advent of Code 2020 - Day 2");

    let input = get_stdin().expect("error reading stdin");

    println!("Input size: {} bytes", input.len());

    let entries = input.parse::<Entries>().expect("invalid entries");

    println!("There are {} entries.", entries.len());

    let count_valid = opt.policy.count_compliant(&entries);

    println!("From those, {} entries are valid.", count_valid);
}

pub fn get_stdin() -> Result<String, io::Error> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    Ok(s)
}
