use crate::*;

#[derive(Clone)]
pub struct GridMap<T> {
    data: HashMap<Position, T>,
    len: usize,
    width: usize,
}

impl<T> FromStr for GridMap<T>
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

impl<T: Debug> Debug for GridMap<T> {
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

impl<T> Index<Position> for GridMap<T> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        self.data.get(&index).ok_or(E::OutOfBounds(index)).unwrap()
    }
}

impl<T> IndexMut<Position> for GridMap<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.data
            .get_mut(&index)
            .ok_or(E::OutOfBounds(index))
            .unwrap()
    }
}

impl<T> Index<Cursor<T>> for GridMap<T> {
    type Output = T;
    fn index(&self, cursor: Cursor<T>) -> &Self::Output {
        self.data
            .get(&cursor.index)
            .ok_or(E::OutOfBounds(cursor.index))
            .unwrap()
    }
}

impl<T> Index<CursorMut<T>> for GridMap<T> {
    type Output = T;
    fn index(&self, cursor: CursorMut<T>) -> &Self::Output {
        self.data
            .get(&cursor.index)
            .ok_or(E::OutOfBounds(cursor.index))
            .unwrap()
    }
}

impl<T> IndexMut<CursorMut<T>> for GridMap<T> {
    fn index_mut(&mut self, cursor: CursorMut<T>) -> &mut Self::Output {
        self.data
            .get_mut(&cursor.index)
            .ok_or(E::OutOfBounds(cursor.index))
            .unwrap()
    }
}

impl<T> GridMap<T> {
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// DEPRECATED. Use `iter_flat`
    pub fn iter(&self) -> impl Iterator<Item = Cursor<T>> + use<'_, T> {
        self.iter_rows().flatten()
    }

    /// DEPRECATED. Use `iter_flat_mut`
    pub fn iter_mut(&mut self) -> impl Iterator<Item = CursorMut<T>> + use<'_, T> {
        self.iter_rows_mut().flatten()
    }

    pub fn iter_flat(&self) -> impl Iterator<Item = Cursor<T>> + use<'_, T> {
        self.iter_rows().flatten()
    }

    pub fn iter_flat_mut(&mut self) -> impl Iterator<Item = CursorMut<T>> + use<'_, T> {
        self.iter_rows_mut().flatten()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = Cursor<T>> + use<'_, T>> {
        (0..self.len).into_iter().map(move |x| {
            (0..self.width)
                .into_iter()
                .map(move |y| Cursor::new(Position::new(x, y), self))
        })
    }

    pub fn iter_rows_mut(
        &mut self,
    ) -> impl Iterator<Item = impl Iterator<Item = CursorMut<T>> + use<'_, T>> {
        let ptr: *mut GridMap<T> = self;
        (0..self.len).into_iter().map(move |x| {
            (0..self.width)
                .into_iter()
                .map(move |y| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = Cursor<T>> + use<'_, T>> {
        (0..self.width).into_iter().map(move |y| {
            (0..self.len)
                .into_iter()
                .map(move |x| Cursor::new(Position::new(x, y), self))
        })
    }

    pub fn iter_cols_mut(
        &mut self,
    ) -> impl Iterator<Item = impl Iterator<Item = CursorMut<T>> + use<'_, T>> {
        let ptr: *mut GridMap<T> = self;
        (0..self.width).into_iter().map(move |y| {
            (0..self.len)
                .into_iter()
                .map(move |x| CursorMut::new(Position::new(x, y), ptr))
        })
    }

    pub fn iter_snake(&self) -> impl Iterator<Item = impl Iterator<Item = Cursor<T>> + use<'_, T>> {
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

    pub fn iter_snake_mut(
        &mut self,
    ) -> impl Iterator<Item = impl Iterator<Item = CursorMut<T>> + use<'_, T>> {
        let ptr: *mut GridMap<T> = self;
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
    pub fn rotate_90(self) -> Self {
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

impl<T: Clone> FromIterator<Cursor<T>> for GridMap<T> {
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
    grid: *const GridMap<T>,
    pub index: Position,
}

pub struct CursorMut<T> {
    grid: *mut GridMap<T>,
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
    fn new(index: Position, grid: *const GridMap<T>) -> Self {
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
    fn new(index: Position, grid: *mut GridMap<T>) -> Self {
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
            .take_if(|next| unsafe { next.x < (*self.grid).len && next.y < (*self.grid).width })
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
            .take_if(|next| unsafe { next.x < (*self.grid).len && next.y < (*self.grid).width })
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
