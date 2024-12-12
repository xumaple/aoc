use std::slice::SliceIndex;

use crate::*;

pub mod position;
pub use position::*;
pub mod direction;
pub use direction::*;
pub mod grid_math;
pub mod grid_map;

#[derive(Clone)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> FromStr for Grid<T>
where
    T: UnsafeFrom<char> + Clone,
{
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.chars().map(T::ufrom).collect_vec())
                .collect_vec(),
        ))
    }
}

impl<T> Grid<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        Self(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn iter_rows_mut(&mut self) -> std::slice::IterMut<Vec<T>> {
        self.0.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut().flatten()
    }

    pub fn flat_enumerate(&self) -> impl Iterator<Item = (PositionPtr<T>, &T)> {
        let width = self.width();
        self.iter()
            .enumerate()
            .map(move |(i, val)| (PositionPtr::new(i / width, i % width, Some(self)), val))
    }

    pub fn flat_enumerate_mut(&mut self) -> impl Iterator<Item = (PositionPtr<T>, &mut T)> {
        let width = self.width();
        let ptr: Option<*const Self> = Some(self);
        self.iter_mut()
            .enumerate()
            .map(move |(i, val)| (PositionPtr::new(i / width, i % width, ptr.clone()), val))
    }

    pub fn enumerate(&self) -> impl Iterator<Item = impl Iterator<Item = (PositionPtr<T>, &T)>> {
        self.iter_rows().enumerate().map(move |(x, v)| {
            v.iter()
                .enumerate()
                .map(move |(y, val)| (PositionPtr::new(x, y, Some(self)), val))
        })
    }

    pub fn into_enumerated(self) -> Grid<PositionT<T>> {
        Grid(
            self.0
                .into_iter()
                .enumerate()
                .map(|(x, v)| {
                    v.into_iter()
                        .enumerate()
                        .map(|(y, val)| (Position::new(x, y), val))
                        .collect_vec()
                })
                .collect_vec(),
        )
    }

    // pub fn pos(&self, position: Position) -> PositionPtr<T> {
    //     PositionPtr::new(position.x, position.y, Some(self))
    // }
}

impl<T: Clone + Default + Copy> Grid<T> {
    pub fn invert(self) -> Self {
        let len = self.len();
        let width = self.width();
        Self(self.0.into_iter().enumerate().fold(
            vec![vec![T::default(); len]; width],
            |mut state, (j, v)| {
                v.iter().enumerate().for_each(|(i, x)| state[i][j] = *x);
                state
            },
        ))
    }

    /// Rotates grid clockwise
    pub fn rotate(self) -> Self {
        Self(
            self.iter_cols()
                .map(|v| v.into_iter().rev().collect_vec())
                .collect_vec(),
        )
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = Vec<T>> {
        self.clone().invert().0.into_iter()
    }

    /// iter diagonally, positive slope:
    /// [ 0, 1, 2,
    ///   3, 4, 5,
    ///   6, 7, 8 ] would yield:
    /// [0],
    /// [3, 1],
    /// [6, 4, 2],
    /// [7, 5],
    /// [8]
    pub fn iter_diags_positive(&self) -> impl Iterator<Item = Vec<T>> + use<'_, T> {
        (0..2 * self.len() - 1).map(|idx| {
            match idx < self.len() {
                true => 0..idx + 1,
                false => idx - self.len() + 1..self.len(),
            }
            .map(|idy| self[idx - idy][idy])
            .collect_vec()
        })
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|line| format!("{:?}", line)).join("\n")
        )
    }
}

impl<T, I: SliceIndex<[Vec<T>]>> Index<I> for Grid<T> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T, I: SliceIndex<[Vec<T>]>> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

impl<'a, T> Index<PositionPtr<T>> for Grid<T> {
    type Output = T;
    fn index(&self, index: PositionPtr<T>) -> &Self::Output {
        &self[index.x][index.y]
    }
}

impl<'a, T> IndexMut<PositionPtr<T>> for Grid<T> {
    fn index_mut(&mut self, index: PositionPtr<T>) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}

impl<'a, T> Index<Position> for Grid<T> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        &self[index.x][index.y]
    }
}

impl<'a, T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_with_dimensions(x: usize, y: usize) -> Self {
        Self(vec![vec![T::default(); y]; x])
    }
}

#[cfg(test)]
mod grid_tests {
    use super::Grid;
    use itertools::Itertools;
    use std::str::FromStr;

    #[test]
    fn iter_diags_positive() {
        let g = Grid::<u32>::from_str("012\n345\n678").unwrap();
        let s = g
            .iter_diags_positive()
            .map(|v| {
                v.into_iter()
                    .map(|i| char::from_digit(i, 10).unwrap())
                    .collect::<String>()
            })
            .join("");
        assert_eq!(s, "031642758");
    }
}
