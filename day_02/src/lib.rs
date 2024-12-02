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
            .map(|l| l.split_ascii_whitespace().map(str::parse).collect())
            .collect::<Result<_, _>>()?;

        Ok(Problem { reports })
    }
}

fn is_safe(record: &[i32]) -> Result<bool, anyhow::Error> {
    ensure!(record.len() > 1, "Record has at least two entries");

    let increasing = record[0] < record[1];

    Ok(record
        .windows(2)
        .map(|levels| levels[0] - levels[1])
        .all(|diff| {
            (increasing && (-3..=-1).contains(&diff)) || (!increasing && (1..=3).contains(&diff))
        }))
}

/// # Panics
///
/// Panics when any of the reports does not met basic assumptions about its format
#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { reports } = p;

    reports
        .iter()
        .filter(|report| is_safe(report).expect("Assumption about record failed"))
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
        assert!(is_safe(&[7, 6, 4, 2, 1]).unwrap());
    }

    #[test]
    fn test_is_safe_2() {
        assert!(!is_safe(&[1, 2, 7, 8, 9]).unwrap());
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 2);
    }
}
