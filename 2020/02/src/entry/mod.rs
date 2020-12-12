pub mod error;

use std::str::FromStr;

use regex::Regex;

use error::*;
use std::ops::Deref;

pub struct Entries(pub Vec<Entry>);

impl Deref for Entries {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Entry {
    pub a: usize,
    pub b: usize,
    pub letter: char,
    pub password: String,
}

impl FromStr for Entry {
    type Err = EntriesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

        let cap = re.captures(s).ok_or("can't parse string")?;

        // safe to use unwrap()s, as the regex is already ensuring correct strings
        let a = (&cap[1]).parse().unwrap();
        let b = (&cap[2]).parse().unwrap();
        let letter = (&cap[3]).chars().next().unwrap();
        let password = (&cap[4]).to_string();

        Ok(Self {
            a,
            b,
            letter,
            password,
        })
    }
}

impl FromStr for Entries {
    type Err = EntriesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();

        for line in s.lines() {
            entries.push(line.parse()?);
        }

        Ok(Self(entries))
    }
}
