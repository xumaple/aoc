use util::*;
#[path = "./shared.rs"]
mod shared;
use shared::*;

#[aoc_proc::aoc_run(06b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let (times, distances) = input.ssplit_once("\n");
    let ans = Race::new(
        times[10..].remove_whitespace().uinto(),
        distances[10..].remove_whitespace().uinto(),
    ).curve_above_distance();
    Ok(ans.end-ans.start)
}
