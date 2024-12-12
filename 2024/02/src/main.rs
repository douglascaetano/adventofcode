use std::cmp::Ordering;
use std::io::Read;
use std::str::FromStr;

const LEVEL_DIFF_THRESHOLD: usize = 3;

#[derive(Debug)]
struct Report {
    levels: Vec<usize>,
}

fn are_levels_safe(levels: &[usize]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let windows = levels.windows(2).map(|slice| match slice {
        [a, b] => (a, b),
        _ => unreachable!(),
    });

    let increasing = levels[0] < levels[1];

    for (a, b) in windows {
        match b.cmp(&a) {
            Ordering::Equal => return false,
            Ordering::Greater if !increasing => return false,
            Ordering::Less if increasing => return false,
            _ if b.abs_diff(*a) > LEVEL_DIFF_THRESHOLD => return false,
            _ => continue,
        }
    }

    true
}

impl Report {
    fn is_safe(&self) -> bool {
        are_levels_safe(&self.levels)
    }

    fn is_safe_with_dampener(&self) -> bool {
        self.is_safe() || {
            (0..self.levels.len())
                .map(|i| {
                    let mut new_levels = self.levels.to_owned();
                    new_levels.remove(i);
                    are_levels_safe(&new_levels)
                })
                .any(|is_safe| is_safe)
        }
    }
}

impl FromStr for Report {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Report { levels })
    }
}

fn count_safe_reports(reports: &[Report]) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

fn count_safe_reports_with_dampener(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe_with_dampener())
        .count()
}

fn main() {
    println!("Hello, Advent of Code 2024!");
    println!("--- Day 2 ---");

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let reports = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .expect("");

    println!(
        "Part 1: there are {} safe reports",
        count_safe_reports(&reports)
    );
    println!(
        "Part 2: there are {} safe reports with dampener",
        count_safe_reports_with_dampener(&reports)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample.txt");

    impl Report {
        fn levels(&self) -> &[usize] {
            &self.levels
        }
    }

    #[test]
    fn test_report_from_str() {
        let report = Report::from_str("1 2 3 4 5").unwrap();

        assert_eq!(report.levels(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reports_are_safe() {
        const REPORTS: &[&str] = &[
            // increasing reports
            "1 2 3 4 5",
            "1 3 4 5",
            "1 4 5",
            // decreasing reports
            "5 4 3 2 1",
            "5 3 2 1",
            "5 2 1",
        ];

        for &report in REPORTS {
            let report = Report::from_str(report).unwrap();
            assert!(report.is_safe(), "report should be safe: {:?}", report);
        }
    }

    #[test]
    fn test_reports_are_not_safe() {
        const REPORTS: &[&str] = &[
            // increasing report by more than three
            "1 2 3 4 10",
            // decreasing report by more than three
            "10 4 3 2 1",
            // increasing and decreasing report
            "1 2 3 5 4",
            // decreasing and increasing report
            "5 4 6 4 1",
        ];

        for &report in REPORTS {
            let report = Report::from_str(report).unwrap();
            assert!(!report.is_safe(), "report should not be safe: {:?}", report);
        }
    }

    #[test]
    fn test_count_safe_reports() {
        let reports: Vec<Report> = SAMPLE_INPUT
            .lines()
            .map(Report::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(count_safe_reports(&reports), 2);
    }

    #[test]
    fn test_reports_are_safe_with_dampener() {
        const REPORTS: &[&str] = &[
            // increasing reports with one unsafe level
            "1 2 3 4 10",
            "1 2 3 5 4",
            // decreasing report with one unsafe level
            "10 4 3 2 1",
            "5 4 6 3 1",
        ];

        for &report in REPORTS {
            let report = Report::from_str(report).unwrap();
            assert!(
                report.is_safe_with_dampener(),
                "report should be safe with dampener: {:?}",
                report
            );
        }
    }

    #[test]
    fn test_reports_are_not_safe_with_dampener() {
        const REPORTS: &[&str] = &[
            // increasing report by more than three
            "1 2 3 9 10",
            // decreasing report by more than three
            "10 9 3 2 1",
            // increasing and decreasing report
            "1 2 6 7 4 5",
            // decreasing and increasing report
            "5 4 7 6 2 1",
        ];

        for &report in REPORTS {
            let report = Report::from_str(report).unwrap();
            assert!(
                !report.is_safe_with_dampener(),
                "report should not be safe with dampener: {:?}",
                report
            );
        }
    }

    #[test]
    fn test_count_safe_reports_with_dampener() {
        let reports: Vec<Report> = SAMPLE_INPUT
            .lines()
            .map(Report::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(count_safe_reports_with_dampener(&reports), 4);
    }
}
