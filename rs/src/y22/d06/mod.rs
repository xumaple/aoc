use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

struct Subroutine(VecDeque<char>, usize);

impl Subroutine {
    pub fn new(size: usize) -> Self {
        Self(VecDeque::new(), size)
    }

    pub fn add(&mut self, new: char) -> &Self {
        self.0.push_back(new);
        if self.0.len() > self.1 {
            self.0.pop_front();
        }
        self
    }

    pub fn is_marker(&self) -> bool {
        HashSet::<char>::from_iter(self.0.clone().into_iter()).len() == self.1
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d06/sample.txt").unwrap()).unwrap(), 7);
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d06/input.txt").unwrap()).unwrap(), 1238);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y22/d06/sample.txt").unwrap()).unwrap(), 19);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y22/d06/input.txt").unwrap()).unwrap(), 3037);
    }
}
