use util::*;
#[path = "./shared.rs"]
mod shared;
use itertools::Itertools;
use shared::*;

pub fn run(filename: &str) -> Result<i64, BoxError> {
    let mut l_iter = read_lines(filename)?;
    let seeds: Vec<i64> = l_iter.next().unwrap()?[7..]
        .split_whitespace()
        .map(i64::ufrom)
        .collect();

    let mappings: Vec<Mapping> = l_iter.fold(Vec::new(), |mut mappings, s| {
        let s = s.unwrap();
        if s.len() == 0 {
            mappings.push(Mapping::new());
            return mappings;
        }
        if !s.chars().next().unwrap().is_digit(10) {
            return mappings;
        }
        mappings
            .last_mut()
            .unwrap()
            .add_map(s.split_whitespace().map(i64::ufrom).collect_vec());
        mappings
    });

    let smallest = seeds
        .iter()
        .map(|&seed| mappings.iter().fold(seed, |seed, map| map.get_map(seed)))
        .min()
        .unwrap();
    Ok(smallest)
}

#[allow(dead_code)]
fn main() -> NulBoxError {
    println!("{}", run("src/d05/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_a {
    use super::run;

    #[test]
    fn sample() {
        assert_eq!(run("src/d05/sample.txt").unwrap(), 35);
    }

    #[test]
    fn offical() {
        assert_eq!(run("src/d05/input.txt").unwrap(), 621354867);
    }
}
