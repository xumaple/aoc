use super::*;

#[aoc_proc::aoc_run(24-12b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<Garden>()?
        .find_sections::<Sides>()
        .map(|section| section.cost())
        .sum();
    Ok(sum)
}
