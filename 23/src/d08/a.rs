use util::*;
#[path = "./shared.rs"]
mod shared;
use shared::*;
use itertools::Itertools;

pub fn run(filename: &str) -> Result<IntType, BoxError> {
    Ok(0)
}

#[allow(dead_code)]
fn main() -> NulBoxError {
    println!("{}", run("src/d08/input.txt")?);
    Ok(())
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
