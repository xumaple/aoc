use super::*;

#[aoc_proc::aoc_run(23-10b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let mut map = input.as_ref().parse::<Map>()?;
    map.get_path_len();
    Ok(map.count_enclosed())
}
