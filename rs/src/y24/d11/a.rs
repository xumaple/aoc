use super::*;

#[aoc_proc::aoc_run(24-11a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let count = (0..25)
        .fold(input.as_ref().parse::<Stones>()?, |stones, _| {
            stones.split()
        })
        .count();
    Ok(count)
}
