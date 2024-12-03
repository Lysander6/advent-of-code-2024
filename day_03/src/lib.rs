use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

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

    fn parse(&mut self) -> Vec<Instruction> {
        let mut result = vec![];

        while self.iterator.peek().is_some() {
            if self.parse_literal("do()") {
                result.push(Instruction::Do);
            } else if self.parse_literal("don't()") {
                result.push(Instruction::Dont);
            } else if self.parse_literal("mul") {
                if self.parse_literal("(") {
                    if let Some(a) = self.parse_number() {
                        if self.parse_literal(",") {
                            if let Some(b) = self.parse_number() {
                                if self.parse_literal(")") {
                                    result.push(Instruction::Mul(a, b));
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

    fn parse_literal(&mut self, literal: &str) -> bool {
        let mut iter = self.iterator.clone();

        for c in literal.chars() {
            if let Some(peeked_c) = iter.peek() {
                if c == *peeked_c {
                    let _ = iter.next();

                    continue;
                }
            }

            return false;
        }

        self.iterator = iter;

        true
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
    let program = parser.parse();

    let mut result = 0;

    for instr in program {
        if let Instruction::Mul(a, b) = instr {
            result += a * b;
        }
    }

    result
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> u64 {
    let Problem { program } = p;
    let mut parser = ProgramParser::new(program);
    let program = parser.parse();

    let mut mul_enabled = true;
    let mut result = 0;

    for instr in program {
        match instr {
            Instruction::Do => {
                mul_enabled = true;
            }
            Instruction::Dont => {
                mul_enabled = false;
            }
            Instruction::Mul(a, b) => {
                if mul_enabled {
                    result += a * b;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let p: Problem = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            .parse()
            .unwrap();

        assert_eq!(solve_part_1(&p), 161);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                .parse()
                .unwrap();

        assert_eq!(solve_part_2(&p), 48);
    }
}
