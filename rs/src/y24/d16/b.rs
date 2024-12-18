use super::*;

#[aoc_proc::aoc_run(24-16b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().parse::<Maze>()?.best_tiles();
    Ok(sum)
}
