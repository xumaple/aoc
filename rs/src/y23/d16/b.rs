use super::*;

#[aoc_proc::aoc_run(23-16b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mirrors = input.as_ref().parse::<Mirrors>()?;
    let width = mirrors.0.width();
    let len = mirrors.0.len();
    Ok((0..width)
        .flat_map(|i| [((0, i), Direction::Down), ((len - 1, i), Direction::Up)])
        .chain((0..len).flat_map(|i| {
            [
                ((i, 0), Direction::Right),
                ((i, width - 1), Direction::Left),
            ]
        }))
        .map(|(coords, dir)| mirrors.clone().energize(coords, dir))
        .max()
        .unwrap())
}
