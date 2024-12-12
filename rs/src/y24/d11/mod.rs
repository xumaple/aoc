use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Debug)]
struct Stones(HashMap<IntType, usize>);

impl FromStr for Stones {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_whitespace_parse::<IntType>().counts()))
    }
}

impl Stones {
    fn split_one(i: IntType) -> Vec<IntType> {
        if i == 0 {
            return vec![1];
        }
        let s = i.to_string();
        if s.len() % 2 == 0 {
            vec![
                s[..s.len() / 2].to_string().uinto(),
                s[s.len() / 2..].to_string().uinto(),
            ]
        } else {
            vec![i * 2024]
        }
    }
    pub fn split(self) -> Self {
        Self(
            self.0
                .into_iter()
                .flat_map(|(stone, count)| {
                    Self::split_one(stone)
                        .into_iter()
                        .map(move |stone| (stone, count))
                })
                .into_grouping_map()
                .sum(),
        )
    }

    pub fn count(self) -> IntType {
        self.0.into_values().sum()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d11/sample.txt").unwrap()).unwrap(), 55312);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d11/input.txt").unwrap()).unwrap(), 203609);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d11/input.txt").unwrap()).unwrap(),
            240954878211138
        );
    }
}
