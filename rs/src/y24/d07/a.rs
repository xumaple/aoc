use super::*;

#[aoc_proc::aoc_run(24-07a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| Calibration::ufrom(line).total_if_can_equal([Op::Add, Op::Mult].into_iter()))
        .sum();
    Ok(sum)
}
