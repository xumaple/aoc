use super::*;

#[aoc_proc::aoc_run(24-10a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().parse::<Topography>()?.trailhead_scores();
    Ok(sum)
}
