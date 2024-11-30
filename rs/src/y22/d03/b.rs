use super::*;

#[aoc_proc::aoc_run(22-03b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .enumerate()
        .chunk_by(|(i, _)| i / 3)
        .into_iter()
        .map(|(_, group)| priority(shared(group.map(|(_, s)| s.to_string()))))
        .sum();
    Ok(sum)
}
