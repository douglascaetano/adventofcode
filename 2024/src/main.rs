fn day1() {
    const INPUT: &str = include_str!("../input/day1_1.txt");

    let (mut list_a, mut list_b) = INPUT.lines().fold(
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
        .into_iter()
        .zip(list_b.into_iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("Part 1: final sum is {}", sum);
}

fn main() {
    println!("Hello, Advent of Code 2024!");

    day1();
}
