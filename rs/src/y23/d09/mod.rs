use util::*;

pub mod a;
pub mod b;

pub type IntType = i64;

pub struct History(Vec<IntType>);

impl FromStr for History {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_whitespace_parse().collect()))
    }
}

impl History {
    pub fn from_history(other: &History) -> Self {
        Self(other.0.windows(2).map(|w| w[1] - w[0]).collect())
    }

    pub fn get_val_a(&self) -> IntType {
        match self.is_zero_aligned() {
            true => 0,
            false => History::from_history(self).get_val_a() + self.0.last().unwrap(),
        }
    }

    pub fn get_val_b(&self) -> IntType {
        match self.is_zero_aligned() {
            true => 0,
            false => self.0.first().unwrap() - History::from_history(self).get_val_b(),
        }
    }

    fn is_zero_aligned(&self) -> bool {
        let (prefix, aligned, suffix) = unsafe { self.0.align_to::<u128>() };

        prefix.iter().all(|&x| x == 0)
            && suffix.iter().all(|&x| x == 0)
            && aligned.iter().all(|&x| x == 0)
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d09/sample.txt").unwrap()).unwrap(), 114);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y23/d09/input.txt").unwrap()).unwrap(),
            2101499000
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d09/sample.txt").unwrap()).unwrap(), 2);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y23/d09/input.txt").unwrap()).unwrap(), 0);
    // }
}
