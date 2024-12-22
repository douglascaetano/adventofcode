mod input_data;

use std::io::Read;
use std::str::FromStr;

use self::input_data::InputData;

fn sum_of_middles<T: AsRef<[usize]>>(pagelists: &[T]) -> usize {
    pagelists.iter().fold(0, |sum, pagelist| {
        let pagelist = pagelist.as_ref();
        sum + pagelist[pagelist.len() / 2]
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 5 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let data = InputData::from_str(&input)?;

    let sum_of_middles_of_correct_updates = sum_of_middles(&data.get_correct_updates());
    let sum_of_middles_of_incorrect_updates = sum_of_middles(&data.fix_incorrect_updates());

    println!("Part 1: sum of middles of correct updates is {sum_of_middles_of_correct_updates}");
    println!(
        "Part 2: sum of middles of incorrect, but fixed, updates is {sum_of_middles_of_incorrect_updates}"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_sum_of_middles() {
        let data = InputData::from_str(SAMPLE).unwrap();
        assert_eq!(sum_of_middles(&data.get_correct_updates()), 143);
    }

    #[test]
    fn test_fix_incorrect_updates() {
        let data = InputData::from_str(SAMPLE).unwrap();
        assert_eq!(sum_of_middles(&data.fix_incorrect_updates()), 123);
    }
}
