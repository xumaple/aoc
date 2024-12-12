use grid_vec::GridVec;
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, Default, Copy, PartialEq)]
pub enum Space {
    Round,
    Cube,
    #[default]
    Empty,
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Round => "O",
                Self::Cube => "#",
                Self::Empty => ".",
            }
        )
    }
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

pub struct Rocks(GridVec<Space>);

impl FromStr for Rocks {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(GridVec::from_str(s)?))
    }
}

impl Debug for Rocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Rocks {
    pub fn load(&self) -> IntType {
        let len = self.0.len();
        self.0
            .iter_rows()
            .enumerate()
            .map(|(i, row)| row.filter(|s| *s.val() == Space::Round).count() * (len - i))
            .sum::<usize>() as IntType
    }

    pub fn total_load_after_rotates(mut self, rotations: IntType) -> IntType {
        let mut c = Cycles::<IntType>::new();
        let cycle_length;
        let mut i = 0;
        let mut load = 0;
        loop {
            for _ in 0..4 {
                let t = self.total_load_mut_then_rotate();
                self = t.0;
                load = t.1;
            }
            i += 1;
            if let Some(l) = c.add(load) {
                cycle_length = l;
                break;
            }
        }
        for _ in 0..(rotations - i) % cycle_length as IntType {
            for _ in 0..4 {
                let t = self.total_load_mut_then_rotate();
                self = t.0;
            }
        }
        self.load()
    }

    pub fn total_load_mut_then_rotate(mut self) -> (Self, IntType) {
        let mut next_curr_lowest = self.0.cursor_mut(Position::new(0, 0));
        let width = self.0.width();
        let load = self
            .0
            .iter_cols_mut()
            .map(|col| {
                let mut curr_lowest = next_curr_lowest.clone();
                let _ = next_curr_lowest.step(Direction::R);
                col.enumerate().fold(0, |acc, (x, mut cs)| match *cs.val() {
                    Space::Round => {
                        if curr_lowest.index != cs.index {
                            std::mem::swap(curr_lowest.val_mut(), cs.val_mut());
                        }
                        let _ = curr_lowest.step(Direction::D);
                        acc + width - x
                    }
                    Space::Cube => {
                        curr_lowest = cs;
                        let _ = curr_lowest.step(Direction::D);
                        acc
                    }
                    Space::Empty => acc,
                })
            })
            .sum();
        (Self(self.0.rotate_90()), load)
    }

    pub fn total_load(&self) -> IntType {
        let len = self.0.len() as IntType;
        self.0
            .iter_cols()
            .map(|col| {
                col.enumerate()
                    .fold(
                        (0 as IntType, 0usize),
                        |(acc, curr_lowest), (i, space)| match *space.val() {
                            Space::Round => (acc + len - curr_lowest as IntType, curr_lowest + 1),
                            Space::Cube => (acc, i + 1),
                            Space::Empty => (acc, curr_lowest),
                        },
                    )
                    .0
            })
            .sum()
    }
}

struct Cycles<T> {
    steps: Vec<T>,
    keys: HashMap<T, Vec<usize>>,
}

impl<T> Cycles<T>
where
    T: Clone + Eq + PartialEq + Hash,
{
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            keys: HashMap::new(),
        }
    }

    pub fn add(&mut self, input: T) -> Option<usize> {
        let entry = self.keys.entry(input.clone()).or_default();
        entry.push(self.steps.len());
        self.steps.push(input);
        let len = entry.len();
        if len >= 3 && entry[len - 1] - entry[len - 2] == entry[len - 2] - entry[len - 3] {
            Some(entry[len - 1] - entry[len - 2])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d14/sample.txt").unwrap()).unwrap(), 136);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d14/input.txt").unwrap()).unwrap(), 109638);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d14/sample.txt").unwrap()).unwrap(), 64);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d14/input.txt").unwrap()).unwrap(), 102657);
    }
}
