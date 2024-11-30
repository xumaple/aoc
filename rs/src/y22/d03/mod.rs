use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Clone, Debug)]
struct RuckSack(String);

impl FromStr for RuckSack {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 1 {
            return Err(E::ParseError);
        }
        Ok(Self(s.to_string()))
    }
}

impl RuckSack {
    pub fn shared(self) -> char {
        let s = &self.0;
        let a = s[..s.len() / 2].to_string();
        let b = s[s.len() / 2..].to_string();
        shared(vec![a, b].into_iter())
    }
}

pub fn shared<I: Iterator<Item = String>>(strings: I) -> char {
    strings
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .reduce(|a, b| &a & &b)
        .unwrap()
        .into_iter()
        .collect_vec()[0]
}

pub fn priority(c: char) -> IntType {
    if c.is_lowercase() {
        IntType::from(c) - IntType::from('a') + 1
    } else {
        IntType::from(c) - IntType::from('A') + 27
    }
}

#[cfg(test)]
mod priority {
    use super::priority;
    #[test]
    fn test() {
        assert_eq!(priority('d'), 4);
        assert_eq!(priority('D'), 30);
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d03/sample.txt").unwrap()).unwrap(), 157);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d03/input.txt").unwrap()).unwrap(), 8240);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d03/sample.txt").unwrap()).unwrap(), 70);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d03/input.txt").unwrap()).unwrap(), 2587);
    }
}
