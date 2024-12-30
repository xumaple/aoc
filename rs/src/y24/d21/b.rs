use super::*;

#[aoc_proc::aoc_run(24-21b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| robot_inputs(line, 26))
        .sum();
    Ok(sum)
}
