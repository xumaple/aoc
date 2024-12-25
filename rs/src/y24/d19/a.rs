use super::*;

#[aoc_proc::aoc_run(24-19a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<TowelsTrie>()?
        .num_patterns_possible();
    Ok(sum)
}
