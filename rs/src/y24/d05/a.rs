use super::*;

#[aoc_proc::aoc_run(24-05a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    Ok(Manual::from_str(input.as_ref()).unwrap().is_right_orders())
}
