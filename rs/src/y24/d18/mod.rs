use abstract_grid::{CursorMut, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Default, Clone, PartialEq, Eq, Copy)]
enum Space {
    #[default]
    Free,
    Corrupted,
}

impl Space {
    pub fn corrupt(&mut self) {
        *self = Self::Corrupted
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Corrupted => '#',
                Self::Free => '.',
            }
        )
    }
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            '#' => Self::Corrupted,
            '.' => Self::Free,
            _ => panic!(),
        }
    }
}

struct Memory {
    grid: Grid<Space>,
    bytes: std::vec::IntoIter<Position>,
    pub last_simulated: Option<Position>,
}

impl Memory {
    pub fn new(s: &str) -> Self {
        Self {
            grid: Grid::new(abstract_grid::GridBoundary::PredefinedSize { len: 71, width: 71 }),
            bytes: s
                .lines()
                .map(|s| {
                    let (x, y) = s.ssplit_once(',');
                    Position::new(x.uinto(), y.uinto())
                })
                .collect_vec()
                .into_iter(),
            last_simulated: None,
        }
    }

    pub fn simulate(mut self, num: usize) -> Self {
        (0..num).for_each(|_| {
            if let Some(pos) = self.bytes.next() {
                self.grid.cursor_mut(pos).val_mut().corrupt();
                self.last_simulated = Some(pos);
            }
        });
        self
    }

    pub fn shortest_path(&mut self) -> IntType {
        astar(Point(self.grid.cursor_mut(Position::new(0, 0)))).1
    }

    pub fn path_exists(&mut self) -> bool {
        astar_safe(Point(self.grid.cursor_mut(Position::new(0, 0)))).is_some()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(CursorMut<Space>);

impl PathfindingNode for Point {
    type Cost = IntType;
    fn is_start(&self) -> bool {
        self.0.index == Position::new(0, 0)
    }
    fn is_goal(&self) -> bool {
        self.0.index == Position::new(self.0.len() - 1, self.0.width() - 1)
    }
    fn next(&self) -> impl Iterator<Item = (Self, Self::Cost)> {
        Direction::iter_clockwise().filter_map(|dir| {
            self.0
                .next(dir)
                .map(|mut cs| (*cs.val_mut() == Space::Free).then(|| (Point(cs), 1)))
                .flatten()
        })
    }
}

impl AStarNode for Point {
    fn heuristic(&self) -> Self::Cost {
        self.0
            .index
            .cardinal_distance(&Position::new(self.0.len() - 1, self.0.width() - 1))
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    // #[test]
    // fn sample() {
    //     assert_eq!(run(read("src/y24/d18/sample.txt").unwrap()).unwrap(), 22);
    // }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d18/input.txt").unwrap()).unwrap(), 292);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d18/sample.txt").unwrap()).unwrap(), "6,1");
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d18/input.txt").unwrap()).unwrap(), 0);
    // }
}
