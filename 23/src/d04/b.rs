use util::*;

pub fn run(filename: &str) -> Result<i32, BoxError> {
    Ok(0)
}

#[allow(dead_code)]
fn main() -> NulBoxError {
    println!("{}", run("src/d04/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_04b {
    use super::run;

    #[test]
    fn sample() {
        assert_eq!(run("src/d04/sample.txt").unwrap(), 0)
    }
}
