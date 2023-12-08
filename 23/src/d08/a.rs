use util::*;
#[path = "./shared.rs"]
mod shared;
use shared::*;

#[aoc_proc::aoc_run(08a)]
pub fn run(input: impl AsRef<str>) -> Result<IntType, BoxError> {
    Ok(0)
}

#[cfg(test)]
mod test_a {
    use super::run;

    #[test]
    fn sample() {
        assert_eq!(run("src/d08/sample.txt").unwrap(), 0);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run("src/d08/input.txt").unwrap(), 0);
    // }
}
