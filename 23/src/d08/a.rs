use super::*;

#[aoc_proc::aoc_run(08a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mut lines = input.as_ref().lines();
    let directions = lines.next().unwrap().parse::<Directions>()?;
    let _ = lines.next();
    let mut map = Map::new();
    // println!("{:?}{:?}{:?}", lines.next(), lines.next(), lines.next());
    map.add_mappings(lines);
    // println!("{:?} {:?}", directions.0.len(), map.indices);
    Ok(map.traverse_steps(directions))
}
