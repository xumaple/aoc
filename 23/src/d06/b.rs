use util::*;
#[path = "./shared.rs"]
mod shared;
use shared::*;

pub fn run(filename: &str) -> Result<IntType, BoxError> {
    let input = read(filename)?;
    let (times, distances) = input.ssplit_once("\n");
    let ans = Race::new(
        times[10..].remove_whitespace().uinto(),
        distances[10..].remove_whitespace().uinto(),
    ).curve_above_distance();
    Ok(ans.end-ans.start)
}

#[allow(dead_code)]
fn main() -> NulBoxError {
    println!("{}", run("src/d06/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_b {
    use super::run;

    #[test]
    fn sample() {
        assert_eq!(run("src/d06/sample.txt").unwrap(), 71503);
    }

    #[test]
    fn offical() {
        assert_eq!(run("src/d06/input.txt").unwrap(), 40087680);
    }
}
