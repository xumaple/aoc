use grid::deprecated::Grid;
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Frequency(char),
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            '.' => Self::Empty,
            a => Self::Frequency(a),
        }
    }
}

struct Antennas(Grid<Space>);

impl FromStr for Antennas {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Grid::from_str(s)?))
    }
}

impl Antennas {
    pub fn get_antinodes(&self, repeat: bool) -> impl Iterator<Item = PositionPtr<Space>> // + use<'_>
    {
        self.0
            .enumerate()
            .flatten()
            .filter_map(|(ptr, &val)| match val {
                Space::Empty => None,
                Space::Frequency(_) => Some(ptr),
            })
            .into_group_map_by(|ptr| ptr.get_val().clone())
            .into_values()
            .map(|v| Frequency::new_from_iter(v.into_iter(), repeat))
            .reduce(|a, b| a + b)
            .unwrap()
            .0
            .into_iter()
    }
}

#[derive(Debug, Clone, Default)]
struct Frequency(HashSet<PositionPtr<Space>>);

impl Frequency {
    fn new_from_iter<T: IntoIterator<Item = PositionPtr<Space>>>(iter: T, repeat: bool) -> Self {
        Self(
            iter.into_iter()
                .combinations(2)
                .map(|v| {
                    Self::get_antinodes_positions(v[0], v[1], repeat).filter(|pos| pos.is_valid())
                })
                .flatten()
                .collect(),
        )
    }

    fn get_antinodes_positions(
        mut a: PositionPtr<Space>,
        mut b: PositionPtr<Space>,
        repeat: bool,
    ) -> impl Iterator<Item = PositionPtr<Space>> {
        let a_diff = a - b;
        let b_diff = b - a;
        let mut yielded = false;
        std::iter::from_fn(move || {
            if !repeat {
                return match yielded {
                    true => None,
                    false => {
                        yielded = true;
                        Some([a + a_diff, b + b_diff].into_iter())
                    }
                };
            }

            if !a.is_valid() && !b.is_valid() {
                return None;
            }
            let next = Some([a, b].into_iter());
            a += a_diff;
            b += b_diff;
            return next;
        })
        .into_iter()
        .flatten()
    }
}

impl std::ops::Add for Frequency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(&self.0 | &rhs.0)
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d08/sample.txt").unwrap()).unwrap(), 14);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d08/input.txt").unwrap()).unwrap(), 318);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d08/sample.txt").unwrap()).unwrap(), 34);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d08/input.txt").unwrap()).unwrap(), 1126);
    }
}
