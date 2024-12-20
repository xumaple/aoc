use super::*;

#[aoc_proc::aoc_run(24-06a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().parse::<Floor>()?.move_guard().count_guard();
    Ok(sum)
}
