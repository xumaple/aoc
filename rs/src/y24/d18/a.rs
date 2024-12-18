use super::*;

#[aoc_proc::aoc_run(24-18a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = Memory::new(input.as_ref()).simulate(1024).shortest_path();
    Ok(sum)
}
