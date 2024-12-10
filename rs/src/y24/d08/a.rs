use super::*;

#[aoc_proc::aoc_run(24-08a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<Antennas>()?
        .get_antinodes(false)
        .count();
    Ok(sum)
}
