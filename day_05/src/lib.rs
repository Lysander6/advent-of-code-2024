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

fn fix_ordering(update: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut pages_before_current_page: HashSet<usize> = HashSet::new();
    let mut update = update.to_vec();
    let mut current_page_ptr = 0;

    while current_page_ptr < update.len() {
        if let Some(pages_that_must_be_after_current_page) = rules.get(&update[current_page_ptr]) {
            let pages_that_must_be_moved_after_current_page = pages_before_current_page
                .intersection(pages_that_must_be_after_current_page)
                .copied()
                .collect::<Vec<_>>();

            let count_of_pages_to_move = pages_that_must_be_moved_after_current_page.len();

            pages_before_current_page.insert(update[current_page_ptr]);

            if count_of_pages_to_move == 0 {
                current_page_ptr += 1;
                continue;
            }

            update = [
                &update[0..(current_page_ptr - count_of_pages_to_move)],
                &[update[current_page_ptr]],
                &pages_that_must_be_moved_after_current_page,
                &update[(current_page_ptr + 1)..],
            ]
            .concat();

            for page in pages_that_must_be_moved_after_current_page {
                pages_before_current_page.remove(&page);
            }

            current_page_ptr -= count_of_pages_to_move - 1;
        } else {
            pages_before_current_page.insert(update[current_page_ptr]);
            current_page_ptr += 1;
        }
    }

    update
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

#[must_use]
pub fn solve_part_2(p: &Problem) -> usize {
    let Problem { rules, updates } = p;

    let invalid_updates = updates
        .iter()
        .filter(|update| !is_valid_ordering(update, rules));

    let fixed_updates = invalid_updates.map(|update| fix_ordering(update, rules));

    let middle_pages = fixed_updates.map(|update| get_middle_page(&update));

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

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 143);
    }

    #[test]
    fn test_page_move() {
        let update = vec![1, 2, 3, 4, 5, 6];
        let current_page_ptr = 3;
        let pages_that_must_be_moved_after_current_page = vec![2, 3];
        let count_of_pages_to_move = pages_that_must_be_moved_after_current_page.len();

        assert_eq!(
            [
                &update[0..(current_page_ptr - count_of_pages_to_move)],
                &[update[current_page_ptr]],
                &pages_that_must_be_moved_after_current_page,
                &update[(current_page_ptr + 1)..],
            ]
            .concat(),
            vec![1, 4, 2, 3, 5, 6]
        );
    }

    #[test]
    fn test_fix_ordering() {
        let Problem { rules, updates } = TEST_INPUT.parse().unwrap();

        // not sure if solution (middle page) depends on order of moved pages, we will find out ¯\_(ツ)_/¯
        // assert_eq!(fix_ordering(&updates[3], &rules), vec![97, 75, 47, 61, 53]);
        // assert_eq!(fix_ordering(&updates[4], &rules), vec![61, 29, 13]);
        // assert_eq!(fix_ordering(&updates[5], &rules), vec![97, 75, 47, 29, 13]);

        assert_eq!(get_middle_page(&fix_ordering(&updates[3], &rules)), 47);
        assert_eq!(get_middle_page(&fix_ordering(&updates[4], &rules)), 29);
        assert_eq!(get_middle_page(&fix_ordering(&updates[5], &rules)), 47);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_2(&p), 123);
    }
}
