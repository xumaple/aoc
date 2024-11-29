use aoc_proc::aoc_run;
use util::*;

#[aoc_run(23-01a)]
pub fn run(input: impl AsRef<str>) -> Result<i32, BoxError> {
    let mut sum = 0;
    for l in input.as_ref().lines() {
        let c1 = l.chars().find(|c| c.is_digit(10));
        let c2 = l.chars().rfind(|c| c.is_digit(10));
        sum += 10 * c1.unwrap().to_digit(10).unwrap() + c2.unwrap().to_digit(10).unwrap();
    }

    Ok(sum.try_into()?)
}
