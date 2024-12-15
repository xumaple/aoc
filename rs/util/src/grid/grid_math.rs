use super::{Position, PositionPtr, SignedPosition};
use std::ops::{Add, AddAssign, Mul, Sub};

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
    }
}

impl<T> Sub for PositionPtr<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.grid, rhs.grid);
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            grid: self.grid,
        }
    }
}

impl<T> Sub<PositionPtr<T>> for Position {
    type Output = PositionPtr<T>;

    fn sub(self, rhs: PositionPtr<T>) -> Self::Output {
        Self::Output {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            grid: rhs.grid,
        }
    }
}

impl<T> Sub<Position> for PositionPtr<T> {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            grid: self.grid,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl<T> Add for PositionPtr<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.grid, rhs.grid);
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            grid: self.grid,
        }
    }
}

impl<T> Add<PositionPtr<T>> for Position {
    type Output = PositionPtr<T>;

    fn add(self, rhs: PositionPtr<T>) -> Self::Output {
        Self::Output {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            grid: rhs.grid,
        }
    }
}

impl<T> Add<Position> for PositionPtr<T> {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            grid: self.grid,
        }
    }
}

impl<T> AddAssign for PositionPtr<T> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.grid, rhs.grid);
        self.x = self.x.wrapping_add(rhs.x);
        self.y = self.y.wrapping_add(rhs.y);
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.wrapping_add(rhs.x);
        self.y = self.y.wrapping_add(rhs.y);
    }
}

impl<T> AddAssign<PositionPtr<T>> for Position {
    fn add_assign(&mut self, rhs: PositionPtr<T>) {
        self.x = self.x.wrapping_add(rhs.x);
        self.y = self.y.wrapping_add(rhs.y);
    }
}

impl<T> AddAssign<Position> for PositionPtr<T> {
    fn add_assign(&mut self, rhs: Position) {
        self.x = self.x.wrapping_add(rhs.x);
        self.y = self.y.wrapping_add(rhs.y);
    }
}

impl Add for SignedPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl AddAssign for SignedPosition {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.wrapping_add(rhs.x);
        self.y = self.y.wrapping_add(rhs.y);
    }
}

impl Add<Position> for SignedPosition {
    type Output = Self;
    fn add(mut self, rhs: Position) -> Self::Output {
        self.x += rhs.x as isize;
        self.y += rhs.y as isize;
        self
    }
}

impl Sub for SignedPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
    }
}

impl Mul<isize> for SignedPosition {
    type Output = Self;
    fn mul(mut self, rhs: isize) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}
