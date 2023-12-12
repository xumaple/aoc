use super::*;

#[aoc_proc::aoc_run(12a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().lines().map(|line| {
        line.parse::<Record>().unwrap().num_combinations()
    }).sum();
    Ok(sum)
}
