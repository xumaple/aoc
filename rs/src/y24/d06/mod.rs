use grid_vec::Grid;
use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Crate([bool; 4]),
    Guard,
}

impl Space {
    pub fn guard_was_here(&self) -> bool {
        match *self {
            Self::Guard => true,
            _ => false,
        }
    }

    pub fn bumped_before(&self, dir: Direction) -> bool {
        match *self {
            Self::Crate(dirs) => dirs[dir as usize],
            _ => false,
        }
    }

    pub fn bump_if_is_crate(&mut self, dir: Direction) -> bool {
        match *self {
            Self::Crate(mut dirs) => {
                dirs[dir as usize] = true;
                *self = Self::Crate(dirs);
                true
            }
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            Self::Empty => true,
            _ => false,
        }
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Crate(_) => '#',
                Self::Empty => '.',
                Self::Guard => '^',
            }
        )
    }
}

impl UnsafeFrom<char> for Space {
    fn ufrom(input: char) -> Self {
        match input {
            '#' => Self::Crate([false; 4]),
            '.' => Self::Empty,
            '^' => Self::Guard,
            _ => panic!(),
        }
    }
}

struct Floor {
    grid: Grid<Space>,
    start: Position,
    paradox_positions: IntType,
}

impl FromStr for Floor {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.parse::<Grid<Space>>()?;
        let start = grid
            .iter_flat()
            .find(|cs| *cs.val() == Space::Guard)
            .unwrap()
            .index;
        Ok(Self {
            grid,
            start,
            paradox_positions: 0,
        })
    }
}

impl Floor {
    pub fn move_guard(mut self) -> Self {
        let mut guard = self.grid.cursor_mut(self.start);
        let mut dir = Direction::U;
        loop {
            *guard.val_mut() = Space::Guard;
            if guard
                .next(dir)
                .map(|mut cs| cs.val_mut().bump_if_is_crate(dir))
                == Some(true)
            {
                dir = dir.turn_R();
            } else if guard.step(dir).is_err() {
                break;
            }

            if let Some(next_space) = guard.next(dir) {
                if next_space.val().is_empty() {
                    let mut new_grid = self.grid.clone();
                    let mut space = Space::Crate([false; 4]);
                    space.bump_if_is_crate(dir);
                    new_grid[next_space.index] = space;
                    if Self::move_until_loop(new_grid, guard.index, dir.turn_R()) {
                        self.paradox_positions += 1;
                    }
                }
            }
        }
        self
    }

    pub fn move_until_loop(mut grid: Grid<Space>, start: Position, mut dir: Direction) -> bool {
        let mut guard = grid.cursor_mut(start);
        loop {
            while let Some(mut next_space) = guard.next(dir) {
                if next_space.val().bumped_before(dir) {
                    return true;
                }
                if next_space.val_mut().bump_if_is_crate(dir) {
                    dir = dir.turn_R();
                } else {
                    break;
                }
            }
            if let Err(_) = guard.step(dir) {
                break;
            }
        }
        false
    }

    pub fn count_guard(self) -> IntType {
        self.grid
            .iter_flat()
            .filter_map(|cs| cs.val().guard_was_here().then_some(()))
            .count()
    }

    pub fn paradoxes(&self) -> IntType {
        self.paradox_positions
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d06/sample.txt").unwrap()).unwrap(), 41);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d06/input.txt").unwrap()).unwrap(), 5331);
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d06/sample.txt").unwrap()).unwrap(), 6);
    }

    #[test]
    fn offical() {
        assert_eq!(run(read("src/y24/d06/input.txt").unwrap()).unwrap(), 1812);
    }
}
