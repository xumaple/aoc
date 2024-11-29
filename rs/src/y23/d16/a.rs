use super::*;

#[aoc_proc::aoc_run(23-16a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mut mirrors = input.as_ref().parse::<Mirrors>()?;
    Ok(mirrors.energize((0, 0), Direction::Right))
}
