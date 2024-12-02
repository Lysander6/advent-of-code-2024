use std::{env, fs};

use anyhow::Context;
use day_02::{solve_part_1, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let input = fs::read_to_string(input_path)?;
    let p: Problem = input.parse()?;

    println!("Part 1: {}", solve_part_1(&p));

    Ok(())
}
