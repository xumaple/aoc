use super::*;

#[aoc_proc::aoc_run(24-03a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().lines().map(|line| mul(line)).sum();
    Ok(sum)
}
