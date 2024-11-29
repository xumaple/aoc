use super::*;

#[aoc_proc::aoc_run(23-12b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            println!("{i}");
            line.parse::<Record>().unwrap().expand(5).num_combinations()
        })
        .sum();
    Ok(sum)
}
