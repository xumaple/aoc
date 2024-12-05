use super::*;

#[aoc_proc::aoc_run(24-05b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    Ok(Manual::from_str(input.as_ref()).unwrap().was_wrong_orders())
}
