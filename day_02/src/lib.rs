use std::str::FromStr;

use anyhow::ensure;

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    reports: Vec<Vec<i32>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s
            .lines()
            .map(|l| {
                let record = l
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?;

                ensure!(record.len() > 1, "Record has at least two entries");

                Ok(record)
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { reports })
    }
}

fn is_safe(record: &[i32]) -> bool {
    let increasing = record[0] < record[1];

    record
        .windows(2)
        .map(|levels| levels[0] - levels[1])
        .all(|diff| {
            (increasing && (-3..=-1).contains(&diff)) || (!increasing && (1..=3).contains(&diff))
        })
}

fn is_safe_with_dampener(record: &[i32]) -> bool {
    if is_safe(record) {
        return true;
    }

    for skip_idx in 0..record.len() {
        if is_safe(&[&record[0..skip_idx], &record[(skip_idx + 1)..]].concat()) {
            return true;
        }
    }

    false
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { reports } = p;

    reports.iter().filter(|report| is_safe(report)).count()
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> usize {
    let Problem { reports } = p;

    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(p.reports.len(), 6);
        assert_eq!(p.reports[0].len(), 5);
    }

    #[test]
    fn test_is_safe_1() {
        assert!(is_safe(&[7, 6, 4, 2, 1]));
    }

    #[test]
    fn test_is_safe_2() {
        assert!(!is_safe(&[1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 2);
    }

    #[test]
    fn test_is_safe_with_dampener_1() {
        assert!(is_safe_with_dampener(&[3, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn test_is_safe_with_dampener_2() {
        assert!(is_safe_with_dampener(&[1, 2, 3, 4, 5, 4]));
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_2(&p), 4);
    }
}
