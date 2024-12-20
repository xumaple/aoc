use super::*;

#[aoc_proc::aoc_run(24-10b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().parse::<Topography>()?.trailhead_ratings();
    Ok(sum)
}
