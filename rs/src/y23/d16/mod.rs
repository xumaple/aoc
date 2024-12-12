use grid_vec::GridVec;
use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Clone, Debug)]
pub struct Beam {
    space: char,
    energized: [bool; 4],
}

impl UnsafeFrom<char> for Beam {
    fn ufrom(input: char) -> Self {
        Self {
            space: input,
            energized: [false; 4],
        }
    }
}

impl Beam {
    pub fn already_energized(&self, direction: &Direction) -> bool {
        self.energized[*direction as usize]
    }

    pub fn bounce(&mut self, direction: Direction) -> (Direction, Option<Direction>) {
        self.energized[direction.clone() as usize] = true;
        match self.space {
            '.' => (direction, None),
            '-' => match direction {
                Direction::D | Direction::U => (Direction::L, Some(Direction::R)),
                _ => (direction, None),
            },
            '|' => match direction {
                Direction::L | Direction::R => (Direction::D, Some(Direction::U)),
                _ => (direction, None),
            },
            '/' => (
                match direction {
                    Direction::L => Direction::D,
                    Direction::R => Direction::U,
                    Direction::D => Direction::L,
                    Direction::U => Direction::R,
                },
                None,
            ),
            '\\' => (
                match direction {
                    Direction::L => Direction::U,
                    Direction::R => Direction::D,
                    Direction::U => Direction::L,
                    Direction::D => Direction::R,
                },
                None,
            ),
            _ => panic!(),
        }
    }

    pub fn is_energized(&self) -> bool {
        self.energized[0] & self.energized[1] & self.energized[2] & self.energized[3]
    }
}

#[derive(Clone)]
pub struct Mirrors(GridVec<Beam>);

impl FromStr for Mirrors {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(GridVec::from_str(s)?))
    }
}

impl Mirrors {
    pub fn energize(&mut self, starting: (usize, usize), starting_dir: Direction) -> IntType {
        let mut stack = VecDeque::new();
        stack.push_back((
            self.0.cursor_mut(Position::new(starting.0, starting.1)),
            starting_dir,
        ));
        while stack.len() > 0 {
            let (mut cursor, dir) = stack.pop_back().unwrap();
            let (dir1, dir2_opt) = cursor.val_mut().bounce(dir);
            if let Some(next_cursor) = cursor.next(dir1) {
                if !next_cursor.val().already_energized(&dir1) {
                    stack.push_back((next_cursor, dir1));
                }
            }
            if let Some(dir2) = dir2_opt {
                if let Some(next_cursor) = cursor.next(dir2) {
                    if !next_cursor.val().already_energized(&dir2) {
                        stack.push_back((next_cursor, dir2));
                    }
                }
            }
        }

        self.total_energized()
    }

    fn total_energized(&self) -> IntType {
        self.0
            .iter()
            .map(|beam| if beam.val().is_energized() { 1 } else { 0 })
            .sum()
    }
}

impl Debug for Mirrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter_rows()
                .map(|row| {
                    row.map(|b| if b.val().is_energized() { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect_vec()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d16/sample.txt").unwrap()).unwrap(), 46);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d16/input.txt").unwrap()).unwrap(), 8389);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y23/d16/sample.txt").unwrap()).unwrap(), 51);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y23/d16/input.txt").unwrap()).unwrap(), 8564);
    }
}
