use crate::*;

pub trait GridTrait<T>: Index<Position> {
    type Cursor: Pointer<T>;
    type CursorMut: PointerMut<T>;

    fn len(&self) -> usize;
    fn width(&self) -> usize;

    fn cursor(&self, index: Position) -> Self::Cursor;
    fn cursor_mut(&mut self, index: Position) -> Self::CursorMut;

    /// DEPRECATED. Use `iter_flat`
    fn iter(&self) -> impl DoubleEndedIterator<Item = Self::Cursor> {
        self.iter_flat()
    }
    /// DEPRECATED. Use `iter_flat_mut`
    fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = Self::CursorMut> {
        self.iter_flat_mut()
    }
    fn iter_flat(&self) -> impl DoubleEndedIterator<Item = Self::Cursor> {
        self.iter_rows().flatten()
    }
    fn iter_flat_mut(&mut self) -> impl DoubleEndedIterator<Item = Self::CursorMut> {
        self.iter_rows_mut().flatten()
    }
    fn iter_rows(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::Cursor>>;
    fn iter_rows_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::CursorMut>>;
    fn iter_cols(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::Cursor>>;
    fn iter_cols_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::CursorMut>>;
    fn iter_snake(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::Cursor>>;
    fn iter_snake_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Self::CursorMut>>;

    fn rotate_90(self) -> Self
    where
        T: Clone + Default + Copy;
}

pub trait Pointer<T>: Copy + Clone + Directional {
    fn index(&self) -> Position;
    fn reset(&mut self) -> &Self;
    fn val(&self) -> &T;
    fn to_enumerated_tuple(&self) -> (Position, T)
    where
        T: Clone;
}

pub trait PointerMut<T>: Pointer<T> {
    fn val_mut(&mut self) -> &mut T;
}
