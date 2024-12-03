use super::*;

#[aoc_proc::aoc_run(24-03b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    Ok(mul_dos(&input.as_ref().lines().join("")))
}
