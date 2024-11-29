use super::*;

#[aoc_proc::aoc_run(23-08b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mut lines = input.as_ref().lines();
    let directions = lines.next().unwrap().parse::<Directions>()?;
    let _ = lines.next();
    let mut map = Map::new();
    let lcm = map
        .add_mappings(
            lines,
            |a| a.chars().nth(2) == Some('A'),
            |z| z.chars().nth(2) == Some('Z'),
        )
        .iter()
        .map(|start| map.traverse_steps(&directions, *start))
        .fold(1, |a, b| lcm(a, b));

    Ok(lcm)
}
