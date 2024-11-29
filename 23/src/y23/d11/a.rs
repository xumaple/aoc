use super::*;

#[aoc_proc::aoc_run(11a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let d = input
        .as_ref()
        .parse::<Universe>()?
        .adjust_positions(2)
        .all_distances_between_galaxies();
    Ok(d)
}
