use super::*;

#[aoc_proc::aoc_run(10b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().lines().map(|line| {
        0
    }).sum();
    Ok(sum)
}
