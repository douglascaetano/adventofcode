const INPUT: &str = include_str!("../input/day1_1.txt");
const SAMPLE: &str = include_str!("../input/day1_sample.txt");

fn day1(input: &str) {
    let (mut list_a, mut list_b) = input.lines().fold(
        (Vec::new(), Vec::new()),
        |(mut list_a, mut list_b), line| {
            let mut values = line.split_whitespace();
            let a = values.next().unwrap();
            let b = values.next().unwrap();

            list_a.push(a.parse::<i32>().unwrap());
            list_b.push(b.parse::<i32>().unwrap());

            (list_a, list_b)
        },
    );

    list_a.sort();
    list_b.sort();

    let sum = list_a
        .iter()
        .zip(list_b.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("Part 1: final sum is {}", sum);

    let score = list_a
        .iter()
        .map(|&location_id| {
            list_b.iter().filter(|&&id| id == location_id).count() as i32 * location_id
        })
        .sum::<i32>();

    println!("Part 2: final score is {}", score);
}

fn main() {
    println!("Hello, Advent of Code 2024!");

    day1(INPUT);
}
