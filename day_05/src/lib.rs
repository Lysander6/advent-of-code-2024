use anyhow::anyhow;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    // `key` must be printed before all the pages in `value`
    rules: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, updates) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Malformed input"))?;

        let rules = rules
            .lines()
            .map(|l| -> Result<(usize, usize), anyhow::Error> {
                let (a, b) = l
                    .split_once('|')
                    .ok_or_else(|| anyhow!("Couldn't split on '|'"))?;

                Ok((a.parse()?, b.parse()?))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let rules = {
            let mut rules_map: HashMap<usize, HashSet<usize>> = HashMap::new();

            for (a, b) in rules {
                rules_map.entry(a).or_default().insert(b);
            }

            rules_map
        };

        let updates = updates
            .lines()
            .map(|l| -> Result<Vec<usize>, anyhow::Error> {
                let pages = l.split(',').map(str::parse).collect::<Result<_, _>>()?;

                Ok(pages)
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { rules, updates })
    }
}

fn is_valid_ordering(update: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut pages_before_current_page: HashSet<usize> = HashSet::new();

    for current_page in update {
        if let Some(pages_that_must_be_after_current_page) = rules.get(current_page) {
            if pages_before_current_page
                .intersection(pages_that_must_be_after_current_page)
                .next()
                .is_some()
            // has at least one element in common
            {
                return false;
            }
        }

        pages_before_current_page.insert(*current_page);
    }

    true
}

fn get_middle_page(update: &[usize]) -> usize {
    update[update.len().div_ceil(2) - 1]
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { rules, updates } = p;

    let valid_updates = updates
        .iter()
        .filter(|update| is_valid_ordering(update, rules));

    let middle_pages = valid_updates.map(|update| get_middle_page(update));

    middle_pages.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(
            p,
            Problem {
                rules: HashMap::from([
                    (47, HashSet::from([53, 13, 61, 29])),
                    (97, HashSet::from([13, 61, 47, 29, 53, 75])),
                    (75, HashSet::from([29, 53, 47, 61, 13])),
                    (61, HashSet::from([13, 53, 29])),
                    (29, HashSet::from([13])),
                    (53, HashSet::from([29, 13])),
                ]),
                updates: vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47],
                ]
            }
        )
    }

    #[test]
    fn test_is_valid_ordering() {
        let Problem { rules, updates } = TEST_INPUT.parse().unwrap();

        assert_eq!(is_valid_ordering(&updates[0], &rules), true);
        assert_eq!(is_valid_ordering(&updates[1], &rules), true);
        assert_eq!(is_valid_ordering(&updates[2], &rules), true);
        assert_eq!(is_valid_ordering(&updates[3], &rules), false);
        assert_eq!(is_valid_ordering(&updates[4], &rules), false);
        assert_eq!(is_valid_ordering(&updates[5], &rules), false);
    }

    #[test]
    fn test_get_middle_page() {
        assert_eq!(get_middle_page(&[1, 2, 3, 4, 5]), 3);
        assert_eq!(get_middle_page(&[4, 9, 7]), 9);
        assert_eq!(get_middle_page(&[3, 6, 1, 4, 7, 9, 2]), 4);
    }
}
