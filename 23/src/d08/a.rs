use super::*;

#[aoc_proc::aoc_run(08a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mut lines = input.as_ref().lines();
    let directions = lines.next().unwrap().parse::<Directions>()?;
    let _ = lines.next();
    let mut map = Map::new();
    let start = map.add_mappings(lines, |a| a=="AAA", |z| z=="ZZZ");
    Ok(map.traverse_steps(&directions, start[0]))
}
