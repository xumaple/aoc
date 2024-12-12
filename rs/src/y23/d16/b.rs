use super::*;

#[aoc_proc::aoc_run(23-16b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mirrors = input.as_ref().parse::<Mirrors>()?;
    let width = mirrors.0.width();
    let len = mirrors.0.len();
    Ok((0..width)
        .flat_map(|i| [((0, i), Direction::D), ((len - 1, i), Direction::U)])
        .chain((0..len).flat_map(|i| [((i, 0), Direction::R), ((i, width - 1), Direction::L)]))
        .map(|(coords, dir)| mirrors.clone().energize(coords, dir))
        .max()
        .unwrap())
}
