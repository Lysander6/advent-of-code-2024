use anyhow::anyhow;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    map: Vec<Vec<u8>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Ok(u8::MAX),
                        c => c
                            .to_digit(10)
                            .map(|d| u8::try_from(d).expect("Couldn't cast to u8"))
                            .ok_or_else(|| anyhow!("Couldn't parse digit from char")),
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { map })
    }
}

fn neighbour_indices(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    vec![
        (x.checked_add_signed(-1), Some(y)),
        (Some(x), y.checked_add_signed(-1)),
        (x.checked_add_signed(1), Some(y)),
        (Some(x), y.checked_add_signed(1)),
    ]
    .into_iter()
    .filter_map(move |(a, b)| match (a, b) {
        (Some(a), Some(b)) if a <= max_x && b <= max_y => Some((a, b)),
        _ => None,
    })
}

fn score_trailhead(map: &[Vec<u8>], starting_point: (usize, usize)) -> usize {
    let max_x = map.len() - 1;
    let max_y = map[0].len() - 1;
    let mut q = VecDeque::from([starting_point]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x, y)) = q.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let height = map[x][y];

        if height == 9 {
            peaks.insert((x, y));

            continue;
        }

        let neighbours = neighbour_indices(x, y, max_x, max_y);

        q.extend(neighbours.filter(|n| map[n.0][n.1] == height + 1 && !visited.contains(n)));
    }

    peaks.len()
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { map } = p;

    let mut score = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                score += score_trailhead(map, (x, y));
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "\
0123
1234
8765
9876";

    const TEST_INPUT_2: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const TEST_INPUT_3: &str = "\
1110111
1111111
1112111
6543456
7111117
8111118
9111119";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT_1.parse().unwrap();

        assert_eq!(p.map.len(), 4);
        assert_eq!(p.map[0].len(), 4);
        assert_eq!(p.map[0][0], 0);
        assert_eq!(p.map[3][3], 6);
        assert_eq!(p.map[2][0], 8);
    }

    #[test]
    fn test_score_tailhead() {
        let p: Problem = TEST_INPUT_1.parse().unwrap();

        assert_eq!(score_trailhead(&p.map, (0, 0)), 1);

        let p: Problem = TEST_INPUT_3.parse().unwrap();

        assert_eq!(score_trailhead(&p.map, (0, 3)), 2);
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT_2.parse().unwrap();

        assert_eq!(solve_part_1(&p), 36);
    }
}
