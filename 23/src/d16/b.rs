use super::*;

#[aoc_proc::aoc_run(16b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mirrors = input.as_ref().parse::<Mirrors>()?;
    let mut starts: Vec<((usize, usize), Direction)> = Vec::new();
    let width = mirrors.0.width();
    let len = mirrors.0.len();
    for i in 0..width {
        starts.push(((0, i), Direction::Down));
        starts.push(((len - 1, i), Direction::Up));
    }
    for i in 0..len {
        starts.push(((i, 0), Direction::Right));
        starts.push(((i, width - 1), Direction::Left))
    }

    Ok(starts
        .iter()
        .map(|&(coords, dir)| mirrors.clone().energize(coords, dir))
        .reduce(|a, b| if a > b { a } else { b })
        .unwrap())
}
