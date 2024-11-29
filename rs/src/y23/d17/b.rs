use super::*;

#[aoc_proc::aoc_run(23-17b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let heat_map = HeatMap::new(input.as_ref(), 4, 10);
    Ok(heat_map.run_algo()?)
}
