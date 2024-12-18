use grid_vec::{Cursor, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, PartialEq, Eq, Copy)]
enum Space {
    Wall,
    Start,
    End,
    Path,
}

impl Space {
    pub fn is_valid(&self) -> bool {
        *self != Self::Wall
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Wall => '#',
                Self::Path => '.',
                Self::Start => 'S',
                Self::End => 'E',
            }
        )
    }
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            '.' => Self::Path,
            _ => panic!(),
        }
    }
}

struct Maze(Grid<Space>);

impl FromStr for Maze {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    cs: Cursor<Space>,
    dir: Direction,
}

impl PathfindingNode for Point {
    type Cost = IntType;
    fn is_start(&self) -> bool {
        *self.cs.val() == Space::Start
    }

    fn is_goal(&self) -> bool {
        *self.cs.val() == Space::End
    }

    fn next(&self) -> impl Iterator<Item = (Self, Self::Cost)> {
        [
            (self.dir, 1),
            (self.dir.turn_L(), 1001),
            (self.dir.turn_R(), 1001),
        ]
        .into_iter()
        .filter_map(move |(dir, cost)| {
            let next_cs = self.cs.next(dir).unwrap();
            next_cs
                .val()
                .is_valid()
                .then(|| (Point::new(next_cs, dir), cost))
        })
    }
}

impl AStarNode for Point {
    fn heuristic(&self) -> Self::Cost {
        self.cs
            .index
            .cardinal_distance(&Position::new(1, self.cs.width() - 2)) as Self::Cost
    }
}

impl Point {
    pub fn new(cs: Cursor<Space>, dir: Direction) -> Self {
        Self { cs, dir }
    }
}

impl Maze {
    pub fn shortest_path_cost(&self) -> IntType {
        astar(self.starting_point()).1
    }

    pub fn best_tiles(&self) -> IntType {
        astar_bag(self.starting_point())
            .0
            .flatten()
            .map(|point| point.cs.index)
            .collect::<HashSet<Position>>()
            .len()
    }

    fn starting_point(&self) -> Point {
        Point::new(
            self.0.cursor(Position::new(self.0.len() - 2, 1)),
            Direction::R,
        )
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d16/sample.txt").unwrap()).unwrap(), 11048);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d16/input.txt").unwrap()).unwrap(), 143564);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d16/sample.txt").unwrap()).unwrap(), 64);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d16/input.txt").unwrap()).unwrap(), 593);
    }
}
