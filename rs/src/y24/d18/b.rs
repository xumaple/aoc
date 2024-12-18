use super::*;

#[aoc_proc::aoc_run(24-18b)]
pub fn run(input: impl AsRef<str>) -> Result<String, BoxError> {
    let mut memory = Memory::new(input.as_ref()).simulate(3000);
    while memory.path_exists() {
        memory = memory.simulate(1);
    }
    let pos = memory.last_simulated.unwrap();
    Ok(format!("{},{}", pos.x, pos.y))
}
