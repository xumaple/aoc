use super::*;

#[aoc_proc::aoc_run(24-21a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| robot_inputs(line, 3))
        .sum();
    Ok(sum)
}
