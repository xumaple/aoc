use super::*;

#[aoc_proc::aoc_run(24-12a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<Garden>()?
        .find_sections::<Perimeter>()
        .map(|section| section.cost())
        .sum();
    Ok(sum)
}
