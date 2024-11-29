use super::*;

#[aoc_proc::aoc_run(15b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mut boxes = Boxes::new();
    input
        .as_ref()
        .split(',')
        .for_each(|sequence| boxes.add(sequence));
    Ok(boxes.sum())
}
