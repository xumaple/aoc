use super::*;

#[aoc_proc::aoc_run(24-18b)]
pub fn run(input: impl AsRef<str>) -> Result<String, BoxError> {
    let pos = Memory::new(input.as_ref())
        .simulate(3010)
        .simulate_until_blocked();
    Ok(format!("{},{}", pos.x, pos.y))
}
