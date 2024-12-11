use std::{iter::successors, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    stones: Vec<usize>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .trim()
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Problem { stones })
    }
}

fn split_if_even_number_of_digits(n: usize) -> Option<(usize, usize)> {
    let nb_of_digits = n.ilog10() + 1;

    if nb_of_digits % 2 != 0 {
        return None;
    }

    Some((
        n / 10usize.pow(nb_of_digits / 2),
        n % 10usize.pow(nb_of_digits / 2),
    ))
}

fn watch_stones(stones: &[usize]) -> Vec<usize> {
    stones
        .iter()
        .flat_map(|&n| {
            if n == 0 {
                return vec![1];
            }

            if let Some((a, b)) = split_if_even_number_of_digits(n) {
                return vec![a, b];
            }

            vec![n * 2024]
        })
        .collect()
}

fn blink(stones: &[usize], times: usize) -> Vec<usize> {
    successors(Some(stones.to_vec()), |stones| Some(watch_stones(stones)))
        .nth(times)
        .expect("Shouldn't happen")
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { stones } = p;

    let stones = blink(stones, 25);

    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 1 10 99 999";
    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(
            p,
            Problem {
                stones: vec![0, 1, 10, 99, 999],
            }
        )
    }

    #[test]
    fn test_if_i_remember_basic_math_correctly() {
        let n = 123456usize;
        let nb_of_digits = n.ilog10() + 1;

        assert_eq!(n / 10usize.pow(nb_of_digits / 2), 123);
        assert_eq!(n % 10usize.pow(nb_of_digits / 2), 456);

        let n = 9999usize;
        let nb_of_digits = n.ilog10() + 1;

        assert_eq!(n / 10usize.pow(nb_of_digits / 2), 99);
        assert_eq!(n % 10usize.pow(nb_of_digits / 2), 99);

        let n = 987001usize;
        let nb_of_digits = n.ilog10() + 1;

        assert_eq!(n / 10usize.pow(nb_of_digits / 2), 987);
        assert_eq!(n % 10usize.pow(nb_of_digits / 2), 1);
    }

    #[test]
    fn test_watch_stones() {
        let stones = vec![0, 1, 10, 99, 999];

        assert_eq!(watch_stones(&stones), vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_blink() {
        assert_eq!(
            blink(&vec![0, 1, 10, 99, 999], 1),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );

        assert_eq!(
            blink(&vec![125, 17], 6),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = "125 17".parse().unwrap();

        assert_eq!(solve_part_1(&p), 55312);
    }
}
