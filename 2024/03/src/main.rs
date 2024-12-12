use std::io::Read;

fn read_mul_pairs(input: &str) -> Vec<(usize, usize)> {
    input
        .split("mul(")
        .skip(1)
        .filter_map(|s| {
            let (pair, _) = s.split_once(')')?;

            let mut iter = pair.split(',');

            let a = iter.next()?.parse().ok()?;
            let b = iter.next()?.parse().ok()?;

            iter.next().is_none().then_some((a, b))
        })
        .collect()
}

fn read_enabled_mul_pairs(input: &str) -> Vec<(usize, usize)> {
    input
        .split("don't()")
        .enumerate()
        .filter_map(|(i, s)| {
            (i == 0)
                .then_some(s)
                .or(s.split_once("do()").map(|(_dont, r#do)| r#do))
        })
        .map(|s| read_mul_pairs(s))
        .flatten()
        .collect()
}

fn sum_of_multiplication(pairs: &[(usize, usize)]) -> usize {
    pairs.iter().map(|(a, b)| a * b).sum()
}

fn main() {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 3 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let pairs = read_mul_pairs(&input);
    let enabled_pairs = read_enabled_mul_pairs(&input);

    println!(
        "Part 1: sum of multiplication is {}",
        sum_of_multiplication(&pairs)
    );
    println!(
        "Part 2: sum of multiplication of enabled pairs is {}",
        sum_of_multiplication(&enabled_pairs)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const SAMPLE_WITH_CONDITIONS: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_read_mul_pairs() {
        let pairs = read_mul_pairs(SAMPLE);

        assert_eq!(pairs.len(), 4);
        assert_eq!(pairs[0], (2, 4));
        assert_eq!(pairs[1], (5, 5));
        assert_eq!(pairs[2], (11, 8));
        assert_eq!(pairs[3], (8, 5));
    }

    #[test]
    fn test_sum_of_multiplication() {
        let pairs = read_mul_pairs(SAMPLE);

        assert_eq!(sum_of_multiplication(&pairs), 161);
    }

    #[test]
    fn test_read_enabled_mul_pairs() {
        let pairs = read_enabled_mul_pairs(SAMPLE_WITH_CONDITIONS);

        assert_eq!(pairs, &[(2, 4), (8, 5)]);
    }

    #[test]
    fn test_sum_of_multiplication_of_enabled_pairs() {
        let pairs = read_enabled_mul_pairs(SAMPLE_WITH_CONDITIONS);

        assert_eq!(sum_of_multiplication(&pairs), 48);
    }
}
