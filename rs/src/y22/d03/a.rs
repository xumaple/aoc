use super::*;

#[aoc_proc::aoc_run(22-03a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(|line| priority(line.parse::<RuckSack>().unwrap().shared()))
        .sum();
    Ok(sum)
}
