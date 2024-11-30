use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Debug, Clone)]
struct Pair(Elf, Elf);

impl FromStr for Pair {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (one, two) = s.ssplit_once(',');
        Ok(Self(one.parse()?, two.parse()?))
    }
}

impl Pair {
    pub fn elf_contains_elf(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn elves_overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

#[derive(Debug, Clone)]
struct Elf(IntType, IntType);

impl FromStr for Elf {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.ssplit_once('-');
        Ok(Self(split.0.uinto(), split.1.uinto()))
    }
}

impl Elf {
    pub fn contains(&self, other: &Elf) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn overlaps(&self, other: &Elf) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d04/sample.txt").unwrap()).unwrap(), 2);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d04/input.txt").unwrap()).unwrap(), 569);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d04/sample.txt").unwrap()).unwrap(), 4);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d04/input.txt").unwrap()).unwrap(), 936);
    }
}
