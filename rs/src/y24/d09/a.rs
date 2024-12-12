use super::*;

#[aoc_proc::aoc_run(24-09a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .parse::<FileSystem>()
        .unwrap()
        .fill_up()
        .checksum();
    Ok(sum)
}
