use super::*;

#[aoc_proc::aoc_run(09b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| line.parse::<History>().unwrap().get_val_b())
        .sum();
    Ok(sum)
}
