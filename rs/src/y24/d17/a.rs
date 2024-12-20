use super::*;

#[aoc_proc::aoc_run(24-17a)]
pub fn run(input: impl AsRef<str>) -> Result<String, BoxError> {
    let sum = input.as_ref().parse::<Computer>()?.execute();
    Ok(sum)
}
