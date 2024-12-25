use super::*;

#[aoc_proc::aoc_run(24-19b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().parse::<TowelsDP>()?.num_paths_possible();
    Ok(sum)
}
