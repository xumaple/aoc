use super::*;

#[aoc_proc::aoc_run(24-01a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let (v1, v2) = input.as_ref().lines().fold(
        (Vec::<IntType>::new(), Vec::<IntType>::new()),
        |(mut v1, mut v2), line| {
            let (s1, s2) = line.ssplit_once("   ");
            v1.push(s1.uinto());
            v2.push(s2.uinto());
            (v1, v2)
        },
    );
    let sum = v1
        .into_iter()
        .sorted()
        .zip(v2.into_iter().sorted())
        .map(|(a, b)| abs_diff(a, b))
        .sum();
    Ok(sum)
}
