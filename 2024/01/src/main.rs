mod ordered_list;

use std::io::Read;

use self::ordered_list::OrderedList;

/// Calculates the total distance between two lists of numbers.
///
/// The lists must be ordered.
fn total_distance(list_a: &OrderedList, list_b: &OrderedList) -> i32 {
    list_a
        .iter()
        .zip(list_b.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs())
}

fn similarity_score(list_a: &[i32], list_b: &[i32]) -> i32 {
    list_a
        .iter()
        .map(|&location_id| {
            list_b.iter().filter(|&&id| id == location_id).count() as i32 * location_id
        })
        .sum()
}

fn split_and_order_lists(input: &str) -> (OrderedList, OrderedList) {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(char::is_whitespace).unwrap();
            (a.parse::<i32>().unwrap(), b.trim().parse::<i32>().unwrap())
        })
        .collect()
}

fn main() {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 1 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (list_a, list_b) = split_and_order_lists(&input);

    println!(
        "Part 1: total distance is {}",
        total_distance(&list_a, &list_b)
    );
    println!(
        "Part 2: similarity score is {}",
        similarity_score(&list_a, &list_b)
    );
}

#[cfg(test)]
mod tests {
    use test;

    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn check_total_distance() {
        let (list_a, list_b) = split_and_order_lists(SAMPLE);
        assert_eq!(total_distance(&list_a, &list_b), 11);
    }

    #[test]
    fn check_similarity_score() {
        let (list_a, list_b) = split_and_order_lists(SAMPLE);
        assert_eq!(similarity_score(&list_a, &list_b), 31);
    }
}
