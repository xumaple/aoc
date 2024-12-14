use super::*;

#[aoc_proc::aoc_run(24-13a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = Regex::new(r"Button A: X\+([0-9]*), Y\+([0-9]*)\nButton B: X\+([0-9]*), Y\+([0-9]*)\nPrize: X=([0-9]*), Y=([0-9]*)").unwrap().captures_iter(input.as_ref()).map(|input| ClawMachine::from_iter(0, input.iter().map(|m| m.unwrap().as_str())).optimal_cost()).sum();
    Ok(sum)
}
