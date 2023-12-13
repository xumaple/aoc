use super::*;

#[aoc_proc::aoc_run(13a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input.as_ref().split("\n\n").map(Mirrors::ufrom).map(|m| {
        m.get_vertical_reflection() as IntType + m.get_horizontal_reflection() as IntType * 100
    }).sum();
    Ok(sum)
}
