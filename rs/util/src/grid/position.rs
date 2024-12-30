use super::{deprecated::Grid, Direction, Directional};
use crate::{abs_diff, UnsafeIntoNum, E};
use std::fmt::Debug;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub type PositionT<T> = (Position, T);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn cardinal_distance(&self, other: &Self) -> usize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }

    pub fn as_signed(&self) -> SignedPosition {
        (*self).into()
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Directional for Position {
    type Err = E;
    fn next(&self, dir: Direction) -> Option<Self> {
        if match dir {
            Direction::U => self.x <= 0,
            Direction::D => false,
            Direction::L => self.y <= 0,
            Direction::R => false,
        } {
            return None;
        }

        match dir {
            Direction::U => Some(Self::new(self.x - 1, self.y)),
            Direction::D => Some(Self::new(self.x + 1, self.y)),
            Direction::L => Some(Self::new(self.x, self.y - 1)),
            Direction::R => Some(Self::new(self.x, self.y + 1)),
        }
    }

    fn move_pos(&self, dist: SignedPosition) -> Option<Self> {
        Some(*self + dist)
    }

    fn error(&self, dir: Direction) -> Self::Err {
        E::OutOfBoundsMove(*self, dir)
    }
}

#[derive(Copy, PartialEq, Eq, Hash)]
pub struct PositionPtr<T> {
    pub x: usize,
    pub y: usize,
    pub grid: Option<*const Grid<T>>,
}

impl<'a, T> Debug for PositionPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<'a, T> PositionPtr<T> {
    pub fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn next(&self, dir: Direction) -> Option<PositionPtr<T>> {
        if match dir {
            Direction::U => self.x <= 0,
            Direction::D => {
                if let Some(len) = self.len() {
                    self.x >= len - 1
                } else {
                    false
                }
            }
            Direction::L => self.y <= 0,
            Direction::R => {
                if let Some(width) = self.width() {
                    self.y >= width - 1
                } else {
                    false
                }
            }
        } {
            return None;
        }

        match dir {
            Direction::U => Some(self.from(self.x - 1, self.y)),
            Direction::D => Some(self.from(self.x + 1, self.y)),
            Direction::L => Some(self.from(self.x, self.y - 1)),
            Direction::R => Some(self.from(self.x, self.y + 1)),
        }
    }

    pub fn from(&self, x: usize, y: usize) -> Self {
        Self::assert_within_range(x, y, self.grid);
        Self {
            x,
            y,
            grid: self.grid,
        }
    }

    pub fn new(x: usize, y: usize, grid: Option<*const Grid<T>>) -> Self {
        Self::assert_within_range(x, y, grid);
        Self { x, y, grid }
    }

    pub fn new_from_tuple(coords: (usize, usize), grid: Option<*const Grid<T>>) -> Self {
        Self::assert_within_range(coords.0, coords.1, grid);
        Self::new(coords.0, coords.1, grid)
    }

    pub fn is_valid(self) -> bool {
        Self::is_within_range(self.x, self.y, self.grid)
    }

    pub fn get_val(&self) -> &T {
        let grid = self.grid.unwrap();
        unsafe { &(*grid)[self.clone()] }
    }

    fn is_within_range(x: usize, y: usize, grid: Option<*const Grid<T>>) -> bool {
        if let Some(grid) = grid {
            unsafe { x < (*grid).len() && y < (*grid).width() }
        } else {
            true
        }
    }

    fn assert_within_range(x: usize, y: usize, grid: Option<*const Grid<T>>) {
        if !Self::is_within_range(x, y, grid) {
            assert!(false)
        }
    }

    fn width(&self) -> Option<usize> {
        if let Some(grid) = self.grid {
            Some(unsafe { (*grid).width() })
        } else {
            None
        }
    }

    fn len(&self) -> Option<usize> {
        if let Some(grid) = self.grid {
            Some(unsafe { (*grid).len() })
        } else {
            None
        }
    }
}

impl<'a, T> Clone for PositionPtr<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            grid: self.grid,
        }
    }
}

impl<T> From<PositionPtr<T>> for Position {
    fn from(value: PositionPtr<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct SignedPosition {
    pub x: isize,
    pub y: isize,
}

impl SignedPosition {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Debug for SignedPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Directional for SignedPosition {
    type Err = E;
    fn next(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::U => Some(Self::new(self.x - 1, self.y)),
            Direction::D => Some(Self::new(self.x + 1, self.y)),
            Direction::L => Some(Self::new(self.x, self.y - 1)),
            Direction::R => Some(Self::new(self.x, self.y + 1)),
        }
    }

    fn move_pos(&self, dist: SignedPosition) -> Option<Self> {
        Some(*self + dist)
    }

    fn error(&self, _: Direction) -> Self::Err {
        unimplemented!()
    }
}

impl From<(isize, isize)> for SignedPosition {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Position> for SignedPosition {
    fn from(value: Position) -> Self {
        Self {
            x: value.x.uinton(),
            y: value.y.uinton(),
        }
    }
}

impl From<SignedPosition> for Position {
    fn from(value: SignedPosition) -> Self {
        Self {
            x: value.x.uinton(),
            y: value.y.uinton(),
        }
    }
}
