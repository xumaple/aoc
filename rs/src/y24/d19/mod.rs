use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Debug)]
struct TowelsTrie {
    available: Trie<u8>,
    patterns: Vec<Vec<u8>>,
}

impl FromStr for TowelsTrie {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (available, patterns) = s.ssplit_once("\n\n");
        Ok(Self {
            available: available
                .split(", ")
                .map(|s| s.to_owned().into_bytes().into_iter())
                .collect(),
            patterns: patterns
                .split('\n')
                .map(|s| s.to_owned().into_bytes())
                .collect_vec(),
        })
    }
}

impl TowelsTrie {
    pub fn num_patterns_possible(&self) -> IntType {
        self.patterns
            .iter()
            .filter(|pattern| self.available.contains(pattern.iter().copied()))
            .count()
    }
}

struct TowelsDP {
    available: MultiMap<usize, String>,
    patterns: Vec<Vec<u8>>,
}

impl FromStr for TowelsDP {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (available, patterns) = s.ssplit_once("\n\n");
        Ok(Self {
            available: available
                .split(", ")
                .map(|s| (s.len(), s.to_owned()))
                .collect(),
            patterns: patterns
                .split('\n')
                .map(|s| s.to_owned().into_bytes())
                .collect_vec(),
        })
    }
}

impl TowelsDP {
    /// About 1 magnitude slower than Trie solution
    // pub fn num_patterns_possible(&self) -> IntType {
    //     self.patterns.iter().filter(|pattern| self.pattern_paths(pattern) > 0).count()
    // }

    fn pattern_paths(&self, pattern: &Vec<u8>) -> IntType {
        let dp = pattern
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut dp, (i, _)| {
                dp.push(
                    self.available
                        .iter()
                        .filter_map(|(len, flags)| {
                            if i + 1 >= *len {
                                flags
                                    .contains(&pattern[i + 1 - len..i + 1].into_string())
                                    .then(|| match i + 1 == *len {
                                        true => 1,
                                        false => dp[i - len],
                                    })
                            } else {
                                None
                            }
                        })
                        .sum(),
                );
                dp
            });

        *dp.last().unwrap()
    }

    pub fn num_paths_possible(&self) -> IntType {
        self.patterns
            .iter()
            .map(|pattern| self.pattern_paths(pattern))
            .sum()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d19/sample.txt").unwrap()).unwrap(), 6);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d19/input.txt").unwrap()).unwrap(), 263);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d19/sample.txt").unwrap()).unwrap(), 16);
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d19/input.txt").unwrap()).unwrap(),
            723524534506343
        );
    }
}
