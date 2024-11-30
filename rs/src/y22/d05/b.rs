use super::*;

#[aoc_proc::aoc_run(22-05b)]
pub fn run(input: impl AsRef<str>) -> Result<String, BoxError> {
    Ok(input.as_ref().parse::<Crates>()?.execute(Order::FIFO).top())
}
