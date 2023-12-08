use util::*;
#[path = "./shared.rs"]
mod shared;
use shared::*;

#[aoc_proc::aoc_run(06a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    let (times, distances) = input.ssplit_once("\n");
    let sum = times[10..].split_whitespace_parse::<IntType>()
        .zip(distances[10..].split_whitespace_parse::<IntType>())
        .map(|(time, distance)| Race::new(time, distance).curve_above_distance())
        .map(|range| range.end-range.start)
        .fold(1, |a, b| a*b);
    Ok(sum)
}

#[cfg(test)]
mod test_a {
    use super::run;

    #[test]
    fn sample() {
        assert_eq!(run("src/d06/sample.txt").unwrap(), 288);
    }

    #[test]
    fn offical() {
        assert_eq!(run("src/d06/input.txt").unwrap(), 1731600);
    }
}
