use std::slice::SliceIndex;

use crate::*;

#[derive(Clone)]
pub struct GridVec<T>(Vec<Vec<T>>);

impl<T> FromStr for GridVec<T>
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

impl<T: Debug> Debug for GridVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|line| format!("{:?}", line)).join("\n")
        )
    }
}

impl<T, I: SliceIndex<[Vec<T>]>> Index<I> for GridVec<T> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T, I: SliceIndex<[Vec<T>]>> IndexMut<I> for GridVec<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

impl<'a, T> Index<Position> for GridVec<T> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        &self[index.x][index.y]
    }
}

impl<'a, T> IndexMut<Position> for GridVec<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}

impl<T> GridVec<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        Self(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn cursor(&self, index: Position) -> Cursor<T> {
        Cursor::new(index, self)
    }

    pub fn cursor_mut(&mut self, index: Position) -> CursorMut<T> {
        CursorMut::new(index, self)
    }

    /// DEPRECATED. Use `iter_flat`
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = Cursor<T>> + use<'_, T> {
        self.iter_rows().flatten()
    }

    /// DEPRECATED. Use `iter_flat_mut`
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = CursorMut<T>> + use<'_, T> {
        self.iter_rows_mut().flatten()
    }

    pub fn iter_flat(&self) -> impl DoubleEndedIterator<Item = Cursor<T>> + use<'_, T> {
        self.iter_rows().flatten()
    }

    pub fn iter_flat_mut(&mut self) -> impl DoubleEndedIterator<Item = CursorMut<T>> + use<'_, T> {
        self.iter_rows_mut().flatten()
    }

    pub fn iter_rows(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Cursor<T>> + use<'_, T>>
    {
        (0..self.len()).into_iter().map(move |x| {
            (0..self.width())
                .into_iter()
                .map(move |y| Cursor::new(Position::new(x, y), self))
        })
    }

    pub fn iter_rows_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = CursorMut<T>> + use<'_, T>>
    {
        let ptr: *mut GridVec<T> = self;
        (0..self.len()).into_iter().map(move |x| {
            (0..self.width())
                .into_iter()
                .map(move |y| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    pub fn iter_cols(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Cursor<T>> + use<'_, T>>
    {
        (0..self.width()).into_iter().map(move |y| {
            (0..self.len())
                .into_iter()
                .map(move |x| Cursor::new(Position::new(x, y), self))
        })
    }

    pub fn iter_cols_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = CursorMut<T>> + use<'_, T>>
    {
        let ptr: *mut GridVec<T> = self;
        (0..self.width()).into_iter().map(move |y| {
            (0..self.len())
                .into_iter()
                .map(move |x| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    pub fn iter_snake(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Cursor<T>> + use<'_, T>>
    {
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

    pub fn iter_snake_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = CursorMut<T>> + use<'_, T>>
    {
        let ptr: *mut GridVec<T> = self;
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
}

impl<T: Clone + Default + Copy> GridVec<T> {
    pub fn rotate_90(self) -> Self {
        Self(
            self.iter_cols()
                .map(|col| col.rev().map(|item| item.val().clone()).collect_vec())
                .collect_vec(),
        )
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

impl<T: Default + Clone> GridVec<T> {
    pub fn new_with_dimensions(x: usize, y: usize) -> Self {
        Self(vec![vec![T::default(); y]; x])
    }
}

#[cfg(test)]
mod grid_tests {
    use super::GridVec;
    use itertools::Itertools;
    use std::str::FromStr;

    #[test]
    fn iter_diags_positive() {
        let g = GridVec::<u32>::from_str("012\n345\n678").unwrap();
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

pub struct Cursor<T> {
    grid: *const GridVec<T>,
    pub index: Position,
}

pub struct CursorMut<T> {
    grid: *mut GridVec<T>,
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
    fn new(index: Position, grid: *const GridVec<T>) -> Self {
        Self { grid, index }
    }

    pub fn reset(&mut self) -> &Self {
        self.index = Position::new(0, 0);
        self
    }

    pub fn val(&self) -> &T {
        unsafe { &(*self.grid)[self.index] }
    }
}

impl<T: Clone> Cursor<T> {
    pub fn to_enumerated_tuple(&self) -> (Position, T) {
        (self.index, self.val().clone())
    }
}

impl<T> CursorMut<T> {
    fn new(index: Position, grid: *mut GridVec<T>) -> Self {
        Self { grid, index }
    }

    pub fn reset(&mut self) -> &Self {
        self.index = Position::new(0, 0);
        self
    }

    pub fn val(&self) -> &T {
        unsafe { &(*self.grid)[self.index] }
    }

    pub fn val_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.grid)[self.index] }
    }
}

impl<T: Clone> CursorMut<T> {
    pub fn to_enumerated_tuple(&self) -> (Position, T) {
        (self.index, self.val().clone())
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

impl<T> From<CursorMut<T>> for Cursor<T> {
    fn from(from: CursorMut<T>) -> Self {
        Self {
            grid: from.grid,
            index: from.index,
        }
    }
}
