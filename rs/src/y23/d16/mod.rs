use util::*;

pub mod a;
pub mod b;

pub type IntType = u32;

#[derive(Clone, Debug, Copy)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn next(
        &self,
        curr: (usize, usize),
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
    ) -> Option<(usize, usize)> {
        if match self {
            Self::Up => curr.0 <= min_x,
            Self::Down => curr.0 >= max_x,
            Self::Left => curr.1 <= min_y,
            Self::Right => curr.1 >= max_y,
        } {
            return None;
        }

        match self {
            Self::Up => Some((curr.0 - 1, curr.1)),
            Self::Down => Some((curr.0 + 1, curr.1)),
            Self::Left => Some((curr.0, curr.1 - 1)),
            Self::Right => Some((curr.0, curr.1 + 1)),
        }
    }
}

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
                Direction::Down | Direction::Up => (Direction::Left, Some(Direction::Right)),
                _ => (direction, None),
            },
            '|' => match direction {
                Direction::Left | Direction::Right => (Direction::Down, Some(Direction::Up)),
                _ => (direction, None),
            },
            '/' => (
                match direction {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Up => Direction::Right,
                },
                None,
            ),
            '\\' => (
                match direction {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                },
                None,
            ),
            _ => panic!(),
        }
    }

    pub fn is_energized(&self) -> bool {
        self.energized.iter().any(|b| *b)
    }
}

#[derive(Clone)]
pub struct Mirrors(Grid<Beam>);

impl FromStr for Mirrors {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Grid::from_str(s)?))
    }
}

impl Mirrors {
    pub fn energize(&mut self, starting: (usize, usize), starting_dir: Direction) -> IntType {
        let mut stack: Vec<((usize, usize), Direction)> = Vec::new();
        stack.push((starting, starting_dir));
        while stack.len() > 0 {
            let (coords, dir) = stack.pop().unwrap();
            let (dir1, dir2_opt) = self.0[coords.0][coords.1].bounce(dir);
            if let Some(coords1) = dir1.next(coords, 0, self.0.len() - 1, 0, self.0.width() - 1) {
                if !self.0[coords1.0][coords1.1].already_energized(&dir1) {
                    stack.push((coords1, dir1));
                }
            }
            if let Some(dir2) = dir2_opt {
                if let Some(coords2) = dir2.next(coords, 0, self.0.len() - 1, 0, self.0.width() - 1)
                {
                    if !self.0[coords2.0][coords2.1].already_energized(&dir2) {
                        stack.push((coords2, dir2));
                    }
                }
            }
        }

        self.total_energized()
    }

    fn total_energized(&self) -> IntType {
        self.0
            .iter()
            .fold(0, |acc, beam| acc + if beam.is_energized() { 1 } else { 0 })
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
                    row.iter()
                        .map(|b| if b.is_energized() { '#' } else { '.' })
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
