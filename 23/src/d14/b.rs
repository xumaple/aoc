use super::*;

#[aoc_proc::aoc_run(14b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let load = input.as_ref().parse::<Rocks>()?.total_load_after_rotates(1000000000);
    Ok(load)
}
