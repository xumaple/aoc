use super::*;

#[aoc_proc::aoc_run(14a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let load = input.as_ref().parse::<Rocks>()?.total_load();
    Ok(load)
}
