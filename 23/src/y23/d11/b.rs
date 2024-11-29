use super::*;

#[aoc_proc::aoc_run(11b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let d = input
        .as_ref()
        .parse::<Universe>()?
        .adjust_positions(1000000)
        .all_distances_between_galaxies();
    Ok(d)
}
