use grid_vec::{Cursor, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Space(char);

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        Self(input)
    }
}

impl Add<u8> for Space {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self((self.0 as u8 + rhs) as char)
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(Cursor<Space>);

impl PathfindingNode for Point {
    type Cost = IntType;
    fn is_goal(&self) -> bool {
        self.0.val().0 == '9'
    }
    fn is_start(&self) -> bool {
        self.0.val().0 == '0'
    }
    fn next(&self) -> impl Iterator<Item = (Self, Self::Cost)> {
        Direction::iter_clockwise().filter_map(|dir| {
            self.0
                .next(dir)
                .map(|cs| (*cs.val() == *self.0.val() + 1).then_some((Point(cs), 1)))
                .flatten()
        })
    }
}

impl AStarNode for Point {
    fn heuristic(&self) -> Self::Cost {
        '9' as Self::Cost - self.0.val().0 as Self::Cost
    }
}

struct Topography(Grid<Space>);

impl FromStr for Topography {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl Topography {
    pub fn trailhead_scores(&self) -> IntType {
        astar_bag_all_starts(self.0.iter_flat().map(|cs| Point(cs)))
            .map(|(_, solutions, _)| {
                solutions
                    .map(|sol| sol.last().unwrap().0.index)
                    .collect::<HashSet<Position>>()
                    .len()
            })
            .sum()
    }

    pub fn trailhead_ratings(&self) -> IntType {
        astar_bag_all_starts(self.0.iter_flat().map(|cs| Point(cs)))
            .map(|(_, solutions, _)| solutions.count())
            .sum()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d10/sample.txt").unwrap()).unwrap(), 36);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d10/input.txt").unwrap()).unwrap(), 694);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d10/sample.txt").unwrap()).unwrap(), 81);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d10/input.txt").unwrap()).unwrap(), 1497);
    }
}
