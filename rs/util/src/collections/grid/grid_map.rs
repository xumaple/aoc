use crate::traits::{GridTrait, Pointer, PointerMut};
use crate::*;

#[derive(Clone)]
pub struct Grid<T> {
    data: HashMap<Position, T>,
    len: usize,
    width: usize,
}

impl<T> FromStr for Grid<T>
where
    T: UnsafeFrom<char> + Clone,
{
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut len = 0;
        let mut width = 0;
        Ok(Self {
            data: s
                .lines()
                .enumerate()
                .flat_map(|(x, line)| {
                    len = x + 1;
                    width = line.len();
                    line.chars()
                        .enumerate()
                        .map(move |(y, c)| (Position::new(x, y), T::ufrom(c)))
                })
                .collect(),
            len,
            width,
        })
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter_rows()
                .map(|row| row.map(|item| format!("{:?}", item.val())).join(""))
                .join("\n"),
        )
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        self.data.get(&index).ok_or(E::OutOfBounds(index)).unwrap()
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.data
            .get_mut(&index)
            .ok_or(E::OutOfBounds(index))
            .unwrap()
    }
}

impl<T> Index<Cursor<T>> for Grid<T> {
    type Output = T;
    fn index(&self, cursor: Cursor<T>) -> &Self::Output {
        self.data
            .get(&cursor.index)
            .ok_or(E::OutOfBounds(cursor.index))
            .unwrap()
    }
}

impl<T> Index<CursorMut<T>> for Grid<T> {
    type Output = T;
    fn index(&self, cursor: CursorMut<T>) -> &Self::Output {
        self.data
            .get(&cursor.index)
            .ok_or(E::OutOfBounds(cursor.index))
            .unwrap()
    }
}

impl<T> IndexMut<CursorMut<T>> for Grid<T> {
    fn index_mut(&mut self, cursor: CursorMut<T>) -> &mut Self::Output {
        self.data
            .get_mut(&cursor.index)
            .ok_or(E::OutOfBounds(cursor.index))
            .unwrap()
    }
}

impl<T> Grid<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        let len = v.len();
        let width = v[0].len();
        Self {
            data: v
                .into_iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(move |(y, item)| (Position::new(x, y), item))
                })
                .collect(),
            len,
            width,
        }
    }
}

impl<T> GridTrait<T> for Grid<T> {
    type Cursor = Cursor<T>;
    type CursorMut = CursorMut<T>;

    fn len(&self) -> usize {
        self.len
    }

    fn width(&self) -> usize {
        self.width
    }

    fn cursor(&self, index: Position) -> Cursor<T> {
        Cursor::new(index, self)
    }

    fn cursor_mut(&mut self, index: Position) -> CursorMut<T> {
        CursorMut::new(index, self)
    }

    /// DEPRECATED. Use `iter_flat`
    fn iter(&self) -> impl DoubleEndedIterator<Item = Cursor<T>> {
        self.iter_rows().flatten()
    }

    /// DEPRECATED. Use `iter_flat_mut`
    fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = CursorMut<T>> {
        self.iter_rows_mut().flatten()
    }

    fn iter_flat(&self) -> impl DoubleEndedIterator<Item = Cursor<T>> {
        self.iter_rows().flatten()
    }

    fn iter_flat_mut(&mut self) -> impl DoubleEndedIterator<Item = CursorMut<T>> {
        self.iter_rows_mut().flatten()
    }

    fn iter_rows(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Cursor<T>>> {
        (0..self.len).into_iter().map(move |x| {
            (0..self.width)
                .into_iter()
                .map(move |y| Cursor::new(Position::new(x, y), self))
        })
    }

    fn iter_rows_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = CursorMut<T>>> {
        let ptr: *mut Grid<T> = self;
        (0..self.len).into_iter().map(move |x| {
            (0..self.width)
                .into_iter()
                .map(move |y| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    fn iter_cols(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Cursor<T>>> {
        (0..self.width).into_iter().map(move |y| {
            (0..self.len)
                .into_iter()
                .map(move |x| Cursor::new(Position::new(x, y), self))
        })
    }

    fn iter_cols_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = CursorMut<T>>> {
        let ptr: *mut Grid<T> = self;
        (0..self.width).into_iter().map(move |y| {
            (0..self.len)
                .into_iter()
                .map(move |x| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    fn iter_snake(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Cursor<T>>> {
        let max_y = self.width - 1;
        (0..self.len).into_iter().map(move |x| {
            (0..self.width).into_iter().map(move |y| {
                Cursor::new(
                    Position::new(x, if x % 2 == 0 { y } else { max_y - y }),
                    self,
                )
            })
        })
    }

    fn iter_snake_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = CursorMut<T>>> {
        let ptr: *mut Grid<T> = self;
        let max_y = self.width - 1;
        (0..self.len).into_iter().map(move |x| {
            (0..self.width).into_iter().map(move |y| {
                CursorMut::new(
                    Position::new(x, if x % 2 == 0 { y } else { max_y - y }),
                    ptr,
                )
            })
        })
    }

    // Rotates 90deg clockwise
    fn rotate_90(self) -> Self {
        let max_x = self.len - 1;
        Self {
            data: self
                .data
                .into_iter()
                .map(|(p, val)| (Position::new(p.y, max_x - p.x), val))
                .collect(),
            len: self.width,
            width: self.len,
        }
    }
}

impl<T> Grid<T> {
    pub fn rotate_180(self) -> Self {
        let max_x = self.len - 1;
        let max_y = self.width - 1;
        Self {
            data: self
                .data
                .into_iter()
                .map(|(p, val)| (Position::new(max_x - p.x, max_y - p.y), val))
                .collect(),
            len: self.width,
            width: self.len,
        }
    }

    // Rotates 270deg clockwise (or 90 deg counterclockwise)
    pub fn rotate_270(self) -> Self {
        let max_y = self.width - 1;
        Self {
            data: self
                .data
                .into_iter()
                .map(|(p, val)| (Position::new(max_y - p.y, p.x), val))
                .collect(),
            len: self.width,
            width: self.len,
        }
    }
}

// impl

impl<T: Clone> FromIterator<Cursor<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Cursor<T>>>(iter: I) -> Self {
        let mut len = 0;
        let mut width = 0;
        Self {
            data: iter
                .into_iter()
                .map(|cs| {
                    len = std::cmp::max(len, cs.index.x);
                    width = std::cmp::max(width, cs.index.y);
                    cs.to_enumerated_tuple()
                })
                .collect(),
            len,
            width,
        }
    }
}

pub struct Cursor<T> {
    grid: *const Grid<T>,
    pub index: Position,
}

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
            .take_if(|next| unsafe { next.x < (*self.grid).len && next.y < (*self.grid).width })
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
            .take_if(|next| unsafe { next.x < (*self.grid).len && next.y < (*self.grid).width })
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

impl<T> From<CursorMut<T>> for Cursor<T> {
    fn from(from: CursorMut<T>) -> Self {
        Self {
            grid: from.grid,
            index: from.index,
        }
    }
}
