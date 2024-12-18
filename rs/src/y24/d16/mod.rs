use grid_vec::{CursorMut, Grid};
use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Clone, PartialEq, Eq, Copy)]
enum Space {
    Wall,
    Path([IntType; 4]),
}

impl Space {
    pub fn traverse_if_valid(&mut self, dir: Direction, score: IntType) -> bool {
        match self {
            Self::Wall => false,
            Self::Path(scores) => {
                let valid = score < scores[dir as usize];
                if valid {
                    scores[dir as usize] = score;
                }
                valid
            }
        }
    }

    fn new_path() -> Self {
        Self::Path([IntType::MAX; 4])
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Wall => '#',
                Self::Path(_) => '.',
            }
        )
    }
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            '#' => Self::Wall,
            'S' => Self::new_path(),
            'E' => Self::new_path(),
            '.' => Self::new_path(),
            _ => panic!(),
        }
    }
}

struct Maze {
    grid: Grid<Space>,
    start: Position,
    end: Position,
}

impl FromStr for Maze {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Space> = s.parse()?;
        let start = Position::new(grid.len() - 2, 1);
        let end = Position::new(1, grid.width() - 2);
        Ok(Self { grid, start, end })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    cs: CursorMut<Space>,
    dir: Direction,
    score: IntType,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score).map(Ordering::reverse)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl Point {
    pub fn new(cs: CursorMut<Space>, dir: Direction, score: IntType) -> Self {
        println!("{cs:?} {dir:?} {score}");
        Self { cs, dir, score }
    }

    pub fn explore(self) -> impl Iterator<Item = Self> {
        [
            (self.dir, 1),
            (self.dir.turn_L(), 1001),
            (self.dir.turn_R(), 1001),
        ]
        .into_iter()
        .filter_map(move |(dir, score)| {
            let mut next_cs = self.cs.next(dir).unwrap();
            next_cs
                .val_mut()
                .traverse_if_valid(dir, self.score + score)
                .then(|| Point::new(next_cs, dir, self.score + score))
        })
    }
}

impl Maze {
    pub fn shortest_path(&mut self) -> IntType {
        let mut pq = BinaryHeap::from([Point::new(
            self.grid.cursor_mut(self.start),
            Direction::R,
            0,
        )]);
        let mut lowest = IntType::MAX;
        while let Some(p) = pq.pop() {
            println!("traversing {:?} {:?} {}", p.cs, p.dir, p.score);

            if p.cs.index == self.end {
                lowest = p.score;
                break;
            }
            pq.extend(p.explore());
        }
        lowest
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
        assert_eq!(run(read("src/y24/d16/sample.txt").unwrap()).unwrap(), 0);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d16/input.txt").unwrap()).unwrap(), 0);
    // }
}
