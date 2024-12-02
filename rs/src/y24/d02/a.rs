use super::*;

#[aoc_proc::aoc_run(24-02a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let sum = Grid::new(
        input
            .as_ref()
            .lines()
            .map(|line| line.split_whitespace_parse::<IntType>().collect_vec())
            .collect_vec(),
    )
    .iter_rows()
    .map(|row| match safe_report(row.iter()) {
        true => 1,
        false => 0,
    })
    .sum();
    Ok(sum)
}
