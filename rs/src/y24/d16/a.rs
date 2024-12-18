use super::*;

#[aoc_proc::aoc_run(24-16a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().parse::<Maze>()?.shortest_path();
    Ok(sum)
}
