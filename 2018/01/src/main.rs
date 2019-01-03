use std::io::{self, Read};
use std::process;
use std::collections::HashSet;

fn main() {
    println!("Advent of Code 2018 - Day 1");
    println!("---------------------------");
    println!();

    let mut input = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut input) {
        eprintln!("Input is not a valid UTF-8 string.");
        process::exit(1);
    }

    let input = from_input(&input).unwrap_or_else(|e| {
        eprintln!("Input line \"{}\" is not a number.", e);
        process::exit(1);
    });

    let ilen = input.len();
    println!("Your input has {} entr{}.", ilen, if ilen == 1 { "y" } else { "ies" });
    println!();

    let ans: i32 = input.iter().sum();

    println!("The sum of the input values is: {}", ans);

    let freq = find_twice(&input);

    println!("The frequency that first occurs twice is: {}", freq);
}

fn from_input(input: &str) -> Result<Vec<i32>, &str> {
    let mut ret = Vec::new();
    for line in input.lines() {
        ret.push(match line.parse() {
            Ok(i) => i,
            Err(_) => return Err(line),
        });
    }
    Ok(ret)
}

fn find_twice(input: &Vec<i32>) -> i32 {
    let mut frequencies = HashSet::new();
    let mut curr_freq = 0;

    frequencies.insert(curr_freq);

    loop {
        for i in input.iter() {
            curr_freq += i;
            if !frequencies.insert(curr_freq) {
                return curr_freq;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpret_input() {
        let input = "+2\n+4\n-3\n";
        let ans = from_input(&input);
        assert_eq!(vec![2, 4, -3], ans.unwrap());
    }

    #[test]
    fn fail_input() {
        let invalid_input = "+2\nwrong\n-3\n";
        assert!(from_input(&invalid_input).is_err());
    }

    #[test]
    fn twice_half_input() {
        let input = vec![2, -5, 3, 7, 9];
        assert_eq!(0, find_twice(&input));
    }

    #[test]
    fn twice_loop_input() {
        let input = vec![1, -2, 3, 1];
        assert_eq!(2, find_twice(&input));
    }
}