use util::*;

fn run(filename: &str) -> Result<i32, BoxError> {
    let mut sum = 0;
    for l in read_lines(filename)? {
        let l = l?;
        let c1 = l.chars().find(|c| c.is_digit(10));
        let c2 = l.chars().rfind(|c| c.is_digit(10));
        sum += 10 * c1.unwrap().to_digit(10).unwrap() + c2.unwrap().to_digit(10).unwrap();
    }

    Ok(sum.try_into()?)
}

fn main() -> NulBoxError {
    println!("{}", run("src/01/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_01a {
    use super::run;

    #[test]
    fn official() {
        assert_eq!(run("src/01/input.txt").unwrap(), 54916)
    }
}
