use util::*;
use std::collections::HashSet;

struct Card {
    winning: HashSet<i32>,
    to_check: HashSet<i32>,
}

impl Card {
    pub fn new(input: &str) -> Self {
        let (winning, to_check) = input.ssplit_once(" | ");
        Self {
            winning: winning.split_whitespace().map(i32::ufrom).collect(),
            to_check: to_check.split_whitespace().map(i32::ufrom).collect(),
        }
    }

    pub fn get_score(&self) -> i32 {
        match self.to_check.intersection(&self.winning).count() {
            0 => 0,
            n => {
                let n: u32 = (n-1).uinton();
                2_i32.pow(n)
            }
        }
    }
}

pub fn run(filename: &str) -> Result<i32, BoxError> {
    let sum = read_lines(filename)?
        .into_iter()
        .map(|line| {
            let c = Card::new(
                line?
                    .ssplit_once(": ")
                    .1,
            );
            Ok(c.get_score())
        })
        .collect::<Result<Vec<i32>, BoxError>>()?
        .iter()
        .sum();
    Ok(sum)
}

#[allow(dead_code)]
fn main() -> NulBoxError {
    println!("{}", run("src/d04/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_04a {
    use super::run;

    #[test]
    fn sample() {
        assert_eq!(run("src/d04/sample.txt").unwrap(), 13);
    }

    #[test]
    fn offical() {
        assert_eq!(run("src/d04/input.txt").unwrap(), 23941);
    }
}
