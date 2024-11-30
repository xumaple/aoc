use super::*;

#[aoc_proc::aoc_run(22-01a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let max = input
        .as_ref()
        .lines()
        .fold((0, 0), |(curr_max, curr_sum), el| {
            if el.len() == 0 {
                (std::cmp::max(curr_max, curr_sum), 0)
            } else {
                (curr_max, curr_sum + IntType::ufrom(el))
            }
        })
        .0;
    Ok(max)
}
