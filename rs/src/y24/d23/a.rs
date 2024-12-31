use super::*;

#[aoc_proc::aoc_run(24-23a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<NetworkMap>()?
        .find_historian_trip_cliques();
    Ok(sum)
}
