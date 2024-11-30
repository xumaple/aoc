use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl UnsafeFrom<char> for Spring {
    fn ufrom(input: char) -> Self {
        match input {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Record(String, Vec<IntType>);

impl FromStr for Record {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, counts) = s.ssplit_once(' ');
        Ok(Self(
            springs.to_string(),
            counts.split(',').map(IntType::ufrom).collect_vec(),
        ))
    }
}

impl Record {
    pub fn expand(self, times: usize) -> Self {
        Self(
            vec![self.0; times].join("?"),
            vec![self.1; times].into_iter().flatten().collect_vec(),
        )
    }

    pub fn num_combinations(&self) -> IntType {
        let mut springs = self.0.chars().map(Spring::ufrom).collect_vec();
        springs.push(Spring::Operational); // For convenience
        let groups = self.1.iter().fold(vec![Spring::Operational; 1], |acc, g| {
            let mut v = vec![Spring::Damaged; g];
            acc.append(&mut v);
            acc.push(Spring::Operational);
            acc
        });
        let n = springs.len();
        let m = groups.len();
        let dp: Vec<Vec<IntType>> = vec![vec![0; n+1]; m+1];
        dp[n][m] = 1;
        for i in n-1..-1
        {
            for j in m-1..-1 {
                let (damaged, operational) = match springs[i] {
                    '#' => (true, false),
                    '.' => (false, true),
                    _ => (true, true)
                };

                let sum = 0;
                if (damaged && groups[])
            }
        }
        0
    }

    // pub fn num_combinations(&self) -> IntType {
    //     self.helper(self.0.clone())
    // }

    // fn helper(&self, springs: String) -> IntType {
    //     let known_counts = Record::get_known_counts(springs.as_str());
    //     if !self.counts_is_possible(&known_counts) {
    //         return 0;
    //     }
    //     let pos = match springs.find('?') {
    //         Some(pos) => pos,
    //         None => { return if known_counts == self.1 { 1 } else { 0 }},
    //     };
    //     let mut s1 = springs.clone();
    //     let mut s2 = springs.clone();
    //     unsafe {
    //         s1.as_mut_vec()[pos] = b'#';
    //         s2.as_mut_vec()[pos] = b'.';
    //     }
    //     self.helper(s1) + self.helper(s2)
    // }

    // fn get_known_counts(s: &str) -> Vec<IntType> {
    //     match s.split_once('?') {
    //         Some((f, _)) => f,
    //         None => s,
    //     }.split('.').filter_map(|s| match s.len() {
    //         0 => None,
    //         i => Some(i as IntType)
    //     }).collect_vec()
    // }

    // fn counts_is_possible(&self, counts: &Vec<IntType>) -> bool {
    //     let len = counts.len();
    //     if len == 0 {
    //         return true;
    //     }
    //     if len > self.1.len() {
    //         return false;
    //     }
    //     if counts[..len - 1] != self.1[..len - 1] {
    //         return false;
    //     }
    //     if counts[len - 1] > self.1[len - 1] {
    //         return false;
    //     }
    //     true
    // }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d12/sample.txt").unwrap()).unwrap(), 21);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d12/input.txt").unwrap()).unwrap(), 7260);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d12/sample.txt").unwrap()).unwrap(), 0);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y23/d12/input.txt").unwrap()).unwrap(), 0);
    // }
}
