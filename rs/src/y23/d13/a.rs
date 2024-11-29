use super::*;

#[aoc_proc::aoc_run(23-13a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .split("\n\n")
        .map(Mirrors::ufrom)
        .map(|m| {
            m.get_vertical_reflection(false)
                .unwrap_or_else(|| m.get_horizontal_reflection(false).unwrap() * 100)
                as IntType
        })
        .sum();
    Ok(sum)
}
