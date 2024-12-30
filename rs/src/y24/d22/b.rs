use super::*;

#[aoc_proc::aoc_run(24-22b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = find_best_price_sequence(input.as_ref().lines().map(|line| IntType::ufrom(line)));
    Ok(sum)
}
