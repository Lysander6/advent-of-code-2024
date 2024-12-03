use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    program: String,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem { program: s.into() })
    }
}

struct ProgramParser<'a> {
    iterator: Peekable<Chars<'a>>,
}

impl<'a> ProgramParser<'a> {
    fn new(program: &'a str) -> Self {
        ProgramParser {
            iterator: program.chars().peekable(),
        }
    }

    fn parse(&mut self) -> u64 {
        let mut result = 0;

        while self.iterator.peek().is_some() {
            if self.parse_mul() {
                if let Some('(') = self.iterator.peek() {
                    let _ = self.iterator.next();

                    if let Some(a) = self.parse_number() {
                        if let Some(',') = self.iterator.peek() {
                            let _ = self.iterator.next();

                            if let Some(b) = self.parse_number() {
                                if let Some(')') = self.iterator.peek() {
                                    let _ = self.iterator.next();

                                    result += a * b;
                                }
                            }
                        }
                    }
                }
            } else {
                let _ = self.iterator.next();
            }
        }

        result
    }

    fn parse_mul(&mut self) -> bool {
        if let Some('m') = self.iterator.peek() {
            let _ = self.iterator.next();
            if let Some('u') = self.iterator.peek() {
                let _ = self.iterator.next();
                if let Some('l') = self.iterator.peek() {
                    let _ = self.iterator.next();

                    return true;
                }
            }
        }

        false
    }

    fn parse_number(&mut self) -> Option<u64> {
        let mut digits = String::new();

        for _ in 0..3 {
            if let Some(c) = self.iterator.peek() {
                if c.is_ascii_digit() {
                    digits.push(*c);

                    let _ = self.iterator.next();

                    continue;
                }
            }

            break;
        }

        if !digits.is_empty() {
            return digits.parse::<u64>().ok();
        }

        None
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> u64 {
    let Problem { program } = p;
    let mut parser = ProgramParser::new(program);

    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 161);
    }
}
