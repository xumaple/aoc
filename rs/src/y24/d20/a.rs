use super::*;

#[aoc_proc::aoc_run(24-20a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    run_race(input, 2, 100)
}
