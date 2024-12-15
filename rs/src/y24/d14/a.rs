use super::*;

#[aoc_proc::aoc_run(24-14a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<Library>()?
        .simulate(100)
        .safety_factor();
    Ok(sum)
}
