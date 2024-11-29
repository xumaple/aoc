use super::*;

#[aoc_proc::aoc_run(23-15a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum: IntType = input
        .as_ref()
        .split(',')
        .map(hash)
        .map(IntType::ufromn)
        .sum();
    Ok(sum)
}
