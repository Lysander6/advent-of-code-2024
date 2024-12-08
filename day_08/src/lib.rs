use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    antennas: HashMap<char, Vec<(usize, usize)>>,
    map_height: usize,
    map_width: usize,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antenna_locations = s.lines().enumerate().flat_map(|(x, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(y, c)| (c != '.').then_some((c, x, y)))
        });

        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for (c, x, y) in antenna_locations {
            antennas.entry(c).or_default().push((x, y));
        }

        let map_height = s.lines().count();
        let map_width = s.lines().next().unwrap().chars().count();

        Ok(Problem {
            antennas,
            map_height,
            map_width,
        })
    }
}

fn get_antinodes(
    a: (usize, usize),
    b: (usize, usize),
    map_height: usize,
    map_width: usize,
) -> Vec<(usize, usize)> {
    // [Option<(usize, usize)>; 2] {
    let mut result = vec![];

    #[allow(clippy::cast_possible_wrap)]
    let ab_diff = (a.0 as isize - b.0 as isize, a.1 as isize - b.1 as isize);
    #[allow(clippy::cast_possible_wrap)]
    let ba_diff = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);

    if let (Some(anti_a_x), Some(anti_a_y)) = (
        a.0.checked_add_signed(ab_diff.0),
        a.1.checked_add_signed(ab_diff.1),
    ) {
        if anti_a_x < map_height && anti_a_y < map_width {
            result.push((anti_a_x, anti_a_y));
        }
    }

    if let (Some(anti_b_x), Some(anti_b_y)) = (
        b.0.checked_add_signed(ba_diff.0),
        b.1.checked_add_signed(ba_diff.1),
    ) {
        if anti_b_x < map_height && anti_b_y < map_width {
            result.push((anti_b_x, anti_b_y));
        }
    }

    result
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem {
        antennas,
        map_height,
        map_width,
    } = p;

    let mut unique_antinode_locations: HashSet<(usize, usize)> = HashSet::new();

    for antennas in antennas.values() {
        let antinodes = antennas
            .iter()
            .cartesian_product(antennas)
            .flat_map(|(a, b)| {
                if a == b {
                    return vec![];
                }

                get_antinodes(*a, *b, *map_height, *map_width)
            });

        unique_antinode_locations.extend(antinodes);
    }

    unique_antinode_locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(p.antennas[&'0'].len(), 4);
        assert_eq!(p.antennas[&'A'].len(), 3);
        assert_eq!(p.antennas[&'0'][0], (1, 8));
        assert_eq!(p.antennas[&'A'][2], (9, 9));
        assert_eq!(p.map_height, 12);
        assert_eq!(p.map_width, 12);
    }

    #[test]
    fn test_get_antinodes() {
        assert_eq!(get_antinodes((3, 4), (5, 5), 12, 12), vec![(1, 3), (7, 6)]);
        assert_eq!(get_antinodes((5, 5), (3, 4), 12, 12), vec![(7, 6), (1, 3)]);
        assert_eq!(get_antinodes((3, 5), (5, 4), 12, 12), vec![(1, 6), (7, 3)]);
        assert_eq!(get_antinodes((5, 4), (3, 5), 12, 12), vec![(7, 3), (1, 6)]);
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 14);
    }
}
