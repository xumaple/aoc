use super::*;

#[aoc_proc::aoc_run(22-04a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = input
        .as_ref()
        .lines()
        .map(
            |line| match line.parse::<Pair>().unwrap().elf_contains_elf() {
                true => 1,
                false => 0,
            },
        )
        .sum();
    Ok(sum)
}
