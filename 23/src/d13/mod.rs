use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

pub struct Mirrors(Vec<Vec<u8>>);

impl FromStr for Mirrors {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|line| line.as_bytes().to_vec()).collect_vec(),
        ))
    }
}

impl Mirrors {
    fn iter_vert<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        self.0
            .iter()
            .map(|v| unsafe { String::from_utf8_unchecked(v.clone()) })
    }

    fn iter_hor(&self) -> impl Iterator<Item = Vec<u8>> + '_ {
        let rotated = self.0.clone().into_iter().enumerate().fold(
            vec![vec![0u8; self.0.len()]; self.0[0].len()],
            |mut state, (j, v)| {
                v.iter().enumerate().for_each(|(i, x)| state[i][j] = *x);
                state
            },
        );
        rotated.into_iter()
    }

    fn equals(s1: &String, s2: &String) -> usize {
        s1.chars()
            .zip(s2.chars())
            .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
            .sum()
    }

    fn get_reflection(v: Vec<String>, fudge: bool) -> Option<usize> {
        for i in 1..v.len() {
            let mut diffs = Mirrors::equals(&v[i], &v[i - 1]);
            if match fudge {
                true => diffs < 2,
                false => diffs == 0,
            } {
                for j in i..v.len() {
                    if 2 * i - j == 1 || j + 1 == v.len() {
                        if fudge && diffs == 0 {
                            break;
                        }
                        return Some(i);
                    }
                    diffs += Mirrors::equals(&v[j + 1], &v[2 * i - j - 2]);
                    if diffs >= if fudge { 2 } else { 1 } {
                        break;
                    }
                }
            }
        }
        None
    }

    pub fn get_vertical_reflection(&self, fudge: bool) -> Option<usize> {
        let iter = self
            .iter_hor()
            .map(|v| unsafe { String::from_utf8_unchecked(v.clone()) });
        Mirrors::get_reflection(iter.collect_vec(), fudge)
    }

    pub fn get_horizontal_reflection(&self, fudge: bool) -> Option<usize> {
        let iter = self.iter_vert();
        Mirrors::get_reflection(iter.collect_vec(), fudge)
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d13/sample.txt").unwrap()).unwrap(), 405);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d13/input.txt").unwrap()).unwrap(), 30487);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/d13/sample.txt").unwrap()).unwrap(), 400);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/d13/input.txt").unwrap()).unwrap(), 31954);
    }
}
