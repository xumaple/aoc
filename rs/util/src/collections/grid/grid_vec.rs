use std::slice::SliceIndex;

use crate::traits::{GridTrait, Pointer, PointerMut};
use crate::*;

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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|line| line.iter().map(|t| format!("{t:?}")).join(""))
                .join("\n")
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

impl<T> Grid<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        Self(v)
    }

    pub fn into_iter_raw(self) -> impl Iterator<Item = impl Iterator<Item = T>> {
        self.0.into_iter().map(|v| v.into_iter())
    }
}

impl<T> GridTrait<T> for Grid<T> {
    type Cursor = Cursor<T>;
    type CursorMut = CursorMut<T>;
    fn len(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn cursor(&self, index: Position) -> Self::Cursor {
        Cursor::new(index, self)
    }

    fn cursor_mut(&mut self, index: Position) -> Self::CursorMut {
        CursorMut::new(index, self)
    }

    fn iter_rows(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::Cursor>> {
        (0..self.len()).into_iter().map(move |x| {
            (0..self.width())
                .into_iter()
                .map(move |y| Cursor::new(Position::new(x, y), self))
        })
    }

    fn iter_rows_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::CursorMut>> {
        let ptr: *mut Grid<T> = self;
        (0..self.len()).into_iter().map(move |x| {
            (0..self.width())
                .into_iter()
                .map(move |y| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    fn iter_cols(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::Cursor>> {
        (0..self.width()).into_iter().map(move |y| {
            (0..self.len())
                .into_iter()
                .map(move |x| Cursor::new(Position::new(x, y), self))
        })
    }

    fn iter_cols_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::CursorMut>> {
        let ptr: *mut Grid<T> = self;
        (0..self.width()).into_iter().map(move |y| {
            (0..self.len())
                .into_iter()
                .map(move |x| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    fn iter_snake(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::Cursor>> {
        let max_y = self.width() - 1;
        (0..self.len()).into_iter().map(move |x| {
            (0..self.width()).into_iter().map(move |y| {
                Cursor::new(
                    Position::new(x, if x % 2 == 0 { y } else { max_y - y }),
                    self,
                )
            })
        })
    }

    fn iter_snake_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::CursorMut>> {
        let ptr: *mut Grid<T> = self;
        let max_y = self.width() - 1;
        (0..self.len()).into_iter().map(move |x| {
            (0..self.width()).into_iter().map(move |y| {
                CursorMut::new(
                    Position::new(x, if x % 2 == 0 { y } else { max_y - y }),
                    ptr,
                )
            })
        })
    }

    fn rotate_90(self) -> Self
    where
        T: Clone + Default + Copy,
    {
        Self(
            self.iter_cols()
                .map(|col| col.rev().map(|item| item.val().clone()).collect_vec())
                .collect_vec(),
        )
    }
}

impl<T: Clone + Default + Copy> Grid<T> {
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

#[derive(PartialEq, Eq)]
pub struct Cursor<T> {
    grid: *const Grid<T>,
    pub index: Position,
}

#[derive(PartialEq, Eq)]
pub struct CursorMut<T> {
    grid: *mut Grid<T>,
    pub index: Position,
}

impl<T> Copy for Cursor<T> {}
impl<T> Copy for CursorMut<T> {}
impl<T> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Clone for CursorMut<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Cursor<T> {
    fn new(index: Position, grid: *const Grid<T>) -> Self {
        Self { grid, index }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.grid).len() }
    }

    pub fn width(&self) -> usize {
        unsafe { (*self.grid).width() }
    }
}

impl<T> Pointer<T> for Cursor<T> {
    fn index(&self) -> Position {
        self.index
    }

    fn reset(&mut self) -> &Self {
        self.index = Position::new(0, 0);
        self
    }

    fn val(&self) -> &T {
        unsafe { &(*self.grid)[self.index] }
    }

    fn to_enumerated_tuple(&self) -> (Position, T)
    where
        T: Clone,
    {
        (self.index, self.val().clone())
    }
}

impl<T> CursorMut<T> {
    fn new(index: Position, grid: *mut Grid<T>) -> Self {
        Self { grid, index }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.grid).len() }
    }

    pub fn width(&self) -> usize {
        unsafe { (*self.grid).width() }
    }
}

impl<T> Pointer<T> for CursorMut<T> {
    fn index(&self) -> Position {
        self.index
    }

    fn reset(&mut self) -> &Self {
        self.index = Position::new(0, 0);
        self
    }

    fn val(&self) -> &T {
        unsafe { &(*self.grid)[self.index] }
    }

    fn to_enumerated_tuple(&self) -> (Position, T)
    where
        T: Clone,
    {
        (self.index, self.val().clone())
    }
}

impl<T> PointerMut<T> for CursorMut<T> {
    fn val_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.grid)[self.index] }
    }
}

impl<T> Directional for Cursor<T> {
    type Err = E;
    fn next(&self, dir: Direction) -> Option<Self> {
        self.index
            .next(dir)
            .take_if(|next| unsafe { next.x < (*self.grid).len() && next.y < (*self.grid).width() })
            .map(|pos| Cursor::new(pos, self.grid))
    }

    fn move_pos(&self, dist: SignedPosition) -> Option<Self> {
        let new_index = self.index + dist;
        unsafe {
            (*self.grid)
                .in_bounds(new_index)
                .then(|| Self::new(new_index, self.grid))
        }
    }

    fn error(&self, dir: Direction) -> Self::Err {
        E::OutOfBoundsMove(self.index, dir)
    }
}

impl<T> Directional for CursorMut<T> {
    type Err = E;
    fn next(&self, dir: Direction) -> Option<Self> {
        self.index
            .next(dir)
            .take_if(|next| unsafe { next.x < (*self.grid).len() && next.y < (*self.grid).width() })
            .map(|pos| CursorMut::new(pos, self.grid))
    }

    fn move_pos(&self, dist: SignedPosition) -> Option<Self> {
        let new_index = self.index + dist;
        unsafe {
            (*self.grid)
                .in_bounds(new_index)
                .then(|| Self::new(new_index, self.grid))
        }
    }

    fn error(&self, dir: Direction) -> Self::Err {
        E::OutOfBoundsMove(self.index, dir)
    }
}

impl<T> Debug for Cursor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.index.fmt(f)
    }
}

impl<T> Debug for CursorMut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.index.fmt(f)
    }
}

impl<T> Hash for Cursor<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<T> Hash for CursorMut<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<T> From<CursorMut<T>> for Cursor<T> {
    fn from(from: CursorMut<T>) -> Self {
        Self {
            grid: from.grid,
            index: from.index,
        }
    }
}
