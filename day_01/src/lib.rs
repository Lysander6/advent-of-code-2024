use anyhow::{anyhow, ensure};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    first_list: Vec<i32>,
    second_list: Vec<i32>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_list, second_list) = s
            .lines()
            .map(|l| -> Result<(i32, i32), anyhow::Error> {
                let (a, b) = l
                    .split_once("   ")
                    .ok_or_else(|| anyhow!("Couldn't split pair by delimiter"))?;

                Ok((a.parse()?, b.parse()?))
            })
            .collect::<Result<(Vec<_>, Vec<_>), _>>()?;

        ensure!(
            first_list.len() == second_list.len(),
            "Lenghts of the lists are not equal"
        );

        Ok(Problem {
            first_list,
            second_list,
        })
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> i32 {
    let Problem {
        first_list,
        second_list,
    } = p;

    let (first_list, second_list) = {
        let mut first_list = first_list.clone();
        let mut second_list = second_list.clone();

        first_list.sort_unstable();
        second_list.sort_unstable();

        (first_list, second_list)
    };

    first_list
        .iter()
        .zip(second_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(
            p,
            Problem {
                first_list: vec![3, 4, 2, 1, 3, 3],
                second_list: vec![4, 3, 5, 3, 9, 3],
            }
        )
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 11);
    }
}
