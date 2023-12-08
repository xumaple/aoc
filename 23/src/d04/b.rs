use std::collections::HashSet;
use util::*;
use aoc_proc::aoc_run;

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

    pub fn num_matches(&self) -> usize {
        self.to_check.intersection(&self.winning).count().uinton()
    }
}

#[aoc_run(04b)]
pub fn run(input: impl AsRef<str>) -> Result<i32, BoxError> {
    let num_matches = input.as_ref().lines()
        .into_iter()
        .map(|line| {
            let c = Card::new(line.ssplit_once(": ").1);
            Ok(c.num_matches())
        })
        .collect::<Result<Vec<usize>, BoxError>>()?;
    let sum = num_matches
        .iter()
        .enumerate()
        .scan(vec![1; num_matches.len()], |v, (i, matches)| {
            for j in i + 1..i + 1 + matches {
                v[j] += v[i];
            }
            Some(v[i])
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod test_0ba {
    use super::run;

    #[test]
    fn sample() {
        assert_eq!(run("src/d04/sample.txt").unwrap(), 30)
    }

    #[test]
    fn offical() {
        assert_eq!(run("src/d04/input.txt").unwrap(), 5571760);
    }
}
