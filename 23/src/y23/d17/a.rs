use super::*;

#[aoc_proc::aoc_run(17a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let heat_map = HeatMap::new(input.as_ref(), 0, 3);
    Ok(heat_map.run_algo()?)
}
