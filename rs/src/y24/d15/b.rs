use super::*;

#[aoc_proc::aoc_run(24-15b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let (s1, s2) = input.as_ref().ssplit_once("\n\n");
    let sum = s1.parse::<Lake>()?.turn_big().move_robot(s2).gps_sum();
    Ok(sum)
}
