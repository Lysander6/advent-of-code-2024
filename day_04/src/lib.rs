use std::str::FromStr;

//   x
//   |
// y-+---->
//   |
//   v

#[derive(Debug)]
pub struct Problem {
    chars: Vec<Vec<char>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.lines().map(|line| line.chars().collect()).collect();

        Ok(Problem { chars })
    }
}

fn count_xmas(chars: &[Vec<char>]) -> usize {
    let mut count = 0;

    let term: Vec<char> = "XMAS".chars().collect();
    let term_rev: Vec<char> = "SAMX".chars().collect();
    let term_len = term.len();

    let chars_height = chars.len();
    let chars_width = chars[0].len();

    for x in 0..chars_height {
        for y in 0..chars_width {
            if (0..=(chars_height - term_len)).contains(&x) {
                // top-down
                if (0..term_len).all(|d| chars[x + d][y] == term[d]) {
                    count += 1;
                }
                // bottom-top
                if (0..term_len).all(|d| chars[x + d][y] == term_rev[d]) {
                    count += 1;
                }
            }

            if (0..=(chars_width - term_len)).contains(&y) {
                // left-right
                if (0..term_len).all(|d| chars[x][y + d] == term[d]) {
                    count += 1;
                }
                // right-left
                if (0..term_len).all(|d| chars[x][y + d] == term_rev[d]) {
                    count += 1;
                }
            }

            if (0..=(chars_height - term_len)).contains(&x)
                && (0..=(chars_width - term_len)).contains(&y)
            {
                // diagonal, top-left to bottom-right
                if (0..term_len).all(|d| chars[x + d][y + d] == term[d]) {
                    count += 1;
                }
                // diagonal, bottom-right to top-left
                if (0..term_len).all(|d| chars[x + d][y + d] == term_rev[d]) {
                    count += 1;
                }
            }

            if (0..=(chars_height - term_len)).contains(&x)
                && ((term_len - 1)..chars_width).contains(&y)
            {
                // diagonal, top-right to bottom-left
                if (0..term_len).all(|d| chars[x + d][y - d] == term[d]) {
                    count += 1;
                }
                // diagonal, bottom-left to top-right
                if (0..term_len).all(|d| chars[x + d][y - d] == term_rev[d]) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_x_mas_duh(chars: &[Vec<char>]) -> usize {
    let mut count = 0;

    let chars_height = chars.len();
    let chars_width = chars[0].len();

    for x in 1..(chars_height - 1) {
        for y in 1..(chars_width - 1) {
            if chars[x][y] == 'A' {
                // M M
                //  A
                // S S
                if (chars[x - 1][y - 1] == 'M' && chars[x + 1][y + 1] == 'S')
                    && (chars[x - 1][y + 1] == 'M' && chars[x + 1][y - 1] == 'S')
                {
                    count += 1;
                }
                // first flipped
                // S M
                //  A
                // S M
                if (chars[x - 1][y - 1] == 'S' && chars[x + 1][y + 1] == 'M')
                    && (chars[x - 1][y + 1] == 'M' && chars[x + 1][y - 1] == 'S')
                {
                    count += 1;
                }
                // second flipped
                // M S
                //  A
                // M S
                if (chars[x - 1][y - 1] == 'M' && chars[x + 1][y + 1] == 'S')
                    && (chars[x - 1][y + 1] == 'S' && chars[x + 1][y - 1] == 'M')
                {
                    count += 1;
                }
                // both flipped
                // S S
                //  A
                // M M
                if (chars[x - 1][y - 1] == 'S' && chars[x + 1][y + 1] == 'M')
                    && (chars[x - 1][y + 1] == 'S' && chars[x + 1][y - 1] == 'M')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { chars } = p;

    count_xmas(chars)
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> usize {
    let Problem { chars } = p;

    count_x_mas_duh(chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_count_xmas() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(count_xmas(&p.chars), 1 + 2 + 3 + 2 + 1 + 4 + 1 + 4);
        assert_eq!(count_xmas(&p.chars), 18);
    }

    #[test]
    fn test_count_x_mas_duh() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(count_x_mas_duh(&p.chars), 1 + 1 + 2 + 5);
        assert_eq!(count_x_mas_duh(&p.chars), 9);
    }
}
