use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("invalid input on stdin.");
    let input = from_input(&input);
    let ans = sum_numbers(&input);
    println!("Ans: {}", ans);
}

fn from_input(input: &str) -> Vec<i32> {
    let mut ret = Vec::new();
    for i in input.lines() {
        ret.push(i.parse().expect(&format!("input line \"{}\" not a number.", &i)));
    }
    ret
}

fn sum_numbers(input: &Vec<i32>) -> i32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpret_input() {
        let input = "+2\n+4\n-3\n";
        let ans = from_input(&input);
        assert_eq!(vec![2, 4, -3], ans);
    }

    #[test]
    #[should_panic]
    fn fail_input() {
        let invalid_input = "+2\nwrong\n-3\n";
        from_input(&invalid_input);
    }

    #[test]
    fn sum_vec() {
        let input = vec![2, 7, -3];
        let ans = sum_numbers(&input);
        assert_eq!(6, ans);
    }
}