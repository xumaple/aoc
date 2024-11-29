use super::*;

#[aoc_proc::aoc_run(23-09a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| line.parse::<History>().unwrap().get_val_a())
        .sum();
    Ok(sum)
}
