use super::*;

#[aoc_proc::aoc_run(23-10a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let length = input.as_ref().parse::<Map>()?.get_path_len();
    Ok(length / 2)
}
