use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct OplessEquation {
    operands: Vec<usize>,
    result: usize,
}

impl FromStr for OplessEquation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, operands) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Couldn't split on ': '"))?;

        let operands = operands
            .split(' ')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let result = result.parse()?;

        Ok(OplessEquation { operands, result })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    equations: Vec<OplessEquation>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations = s.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Problem { equations })
    }
}

fn try_operation(val: usize, remaining: &[usize], expected: usize) -> bool {
    if remaining.is_empty() {
        return val == expected;
    }

    try_operation(val + remaining[0], &remaining[1..], expected)
        || try_operation(val * remaining[0], &remaining[1..], expected)
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { equations } = p;

    equations
        .iter()
        .filter_map(|OplessEquation { operands, result }| {
            if try_operation(operands[0], &operands[1..], *result) {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(p.equations.len(), 9);
        assert_eq!(
            p.equations[0],
            OplessEquation {
                operands: vec![10, 19],
                result: 190,
            }
        );

        assert_eq!(
            p.equations[8],
            OplessEquation {
                operands: vec![11, 6, 16, 20],
                result: 292,
            }
        );
    }

    #[test]
    fn test_try_operation() {
        assert_eq!(try_operation(10, &[19], 190), true);
        assert_eq!(try_operation(81, &[40, 27], 3267), true);
        assert_eq!(try_operation(11, &[6, 16, 20], 292), true);
        assert_eq!(try_operation(17, &[5], 83), false);
        assert_eq!(try_operation(16, &[10, 13], 161011), false);
        assert_eq!(try_operation(9, &[7, 18, 13], 21037), false);
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 3749);
    }
}