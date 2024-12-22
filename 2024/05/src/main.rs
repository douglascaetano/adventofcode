mod input_data;

use std::io::Read;
use std::str::FromStr;

use self::input_data::InputData;

fn sum_of_middles(pagelists: &[&[usize]]) -> usize {
    pagelists
        .iter()
        .fold(0, |sum, pagelist| sum + pagelist[pagelist.len() / 2])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 5 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let data = InputData::from_str(&input)?;

    let sum_of_middles = sum_of_middles(&data.get_correct_updates());

    println!("Part 1: sum of middles of correct updates is {sum_of_middles}");
    println!("Part 2: ?");

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
}
