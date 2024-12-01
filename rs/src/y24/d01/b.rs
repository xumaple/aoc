use super::*;

#[aoc_proc::aoc_run(24-01b)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let (counts, els) = input.as_ref().lines().fold(
        (HashMap::<IntType, IntType>::new(), Vec::<IntType>::new()),
        |(mut map, mut v), line| {
            let (s1, s2) = line.ssplit_once("   ");
            v.push(s1.uinto());
            *map.entry(s2.uinto()).or_default() += 1;
            (map, v)
        },
    );
    let sum = els
        .iter()
        .map(|&n| match counts.get(&n) {
            Some(m) => n * m,
            None => 0,
        })
        .sum();
    Ok(sum)
}
