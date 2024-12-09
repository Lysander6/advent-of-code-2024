use anyhow::anyhow;
use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum DiskMapEntry {
    File { id: usize, length: usize },
    FreeSpace(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    disk_map: Vec<DiskMapEntry>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let disk_map = s
            .trim()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let length = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("Couldn't parse a digit from char"))?
                    as usize;

                if i % 2 == 0 {
                    Ok(DiskMapEntry::File { id: i / 2, length })
                } else {
                    Ok(DiskMapEntry::FreeSpace(length))
                }
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Problem { disk_map })
    }
}

fn compact_disk(disk_map: &[DiskMapEntry]) -> Vec<usize> {
    let mut disk_image = Vec::new();

    let mut data_blocks_rev = disk_map
        .iter()
        .rev()
        .filter_map(|entry| match entry {
            &DiskMapEntry::File { id, length } => Some(vec![id; length]),
            DiskMapEntry::FreeSpace(_) => None,
        })
        .flatten();

    let mut last_processed_file_id: Option<usize> = None;

    for entry in disk_map {
        match *entry {
            DiskMapEntry::File { id, length } => {
                if last_processed_file_id.is_some_and(|last_file_id| last_file_id == id) {
                    break;
                }

                disk_image.extend(repeat(id).take(length));
                last_processed_file_id = Some(id);
            }
            DiskMapEntry::FreeSpace(length) => {
                disk_image.extend(
                    data_blocks_rev
                        .by_ref()
                        .take(length)
                        .take_while(|id| *id != last_processed_file_id.unwrap()),
                );

                last_processed_file_id = Some(disk_image[disk_image.len() - 1]);
            }
        }
    }

    disk_image.extend(data_blocks_rev.take_while(|id| *id == last_processed_file_id.unwrap()));

    disk_image
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { disk_map } = p;

    let disk_image = compact_disk(disk_map);

    disk_image
        .into_iter()
        .enumerate()
        .map(|(i, id)| i * id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use DiskMapEntry::{File, FreeSpace};

    const TEST_INPUT_1: &str = "12345";

    const TEST_INPUT_2: &str = "2333133121414131402";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT_1.parse().unwrap();

        assert_eq!(
            p.disk_map,
            vec![
                File { id: 0, length: 1 },
                FreeSpace(2),
                File { id: 1, length: 3 },
                FreeSpace(4),
                File { id: 2, length: 5 },
            ]
        );

        let p: Problem = TEST_INPUT_2.parse().unwrap();

        assert_eq!(p.disk_map.len(), 19);
    }

    #[test]
    fn test_compact_disk() {
        let p: Problem = TEST_INPUT_1.parse().unwrap();

        assert_eq!(compact_disk(&p.disk_map), vec![0, 2, 2, 1, 1, 1, 2, 2, 2]);

        let p: Problem = TEST_INPUT_2.parse().unwrap();

        assert_eq!(
            compact_disk(&p.disk_map),
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        );
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT_2.parse().unwrap();

        assert_eq!(solve_part_1(&p), 1928);
    }
}
