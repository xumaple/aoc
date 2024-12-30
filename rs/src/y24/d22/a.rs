use super::*;

#[aoc_proc::aoc_run(24-22a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| iterate(IntType::ufrom(line), 2000))
        .sum();
    Ok(sum)
}
