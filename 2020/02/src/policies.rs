use std::str::FromStr;

use crate::entry::{Entries, Entry};

#[derive(Debug)]
pub enum PolicyType {
    New,
    Old,
}

impl FromStr for PolicyType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "new" => Ok(Self::New),
            "old" => Ok(Self::Old),
            _ => Err("invalid policy type"),
        }
    }
}

impl PolicyType {
    pub fn count_compliant(&self, entries: &Entries) -> usize {
        entries
            .iter()
            .filter(match self {
                Self::New => |e: &&Entry| Self::is_entry_compliant_new(*e),
                Self::Old => |e: &&Entry| Self::is_entry_compliant_old(*e),
            })
            .count()
    }

    fn is_entry_compliant_new(entry: &Entry) -> bool {
        eprint!("{:?} = ", entry);
        let pair = {
            let mut chars = entry.password.chars();
            (
                chars
                    .nth(entry.a - 1)
                    .expect("password for this entry wasn't long enough"),
                chars.nth(entry.b - entry.a - 1),
            )
        };

        let r = match pair {
            // both letters can't be equal
            (first, Some(second)) if first == second => false,

            // either one of the positions must contain the letter
            (first, _) if first == entry.letter => true,
            (_, Some(second)) if second == entry.letter => true,

            // neither positions contain the letter
            _ => false,
        };

        eprintln!("{:?}", r);
        r
    }

    fn is_entry_compliant_old(entry: &Entry) -> bool {
        (entry.a..=entry.b).contains(
            &entry
                .password
                .chars()
                .filter(|&ch| ch == entry.letter)
                .count(),
        )
    }
}
