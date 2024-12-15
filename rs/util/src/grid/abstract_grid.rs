use crate::traits::{GridTrait, Pointer, PointerMut};
use crate::*;

pub enum GridBoundary {
    /// Endless boundary. Grid can continue endlessly.
    Endless,
    PredefinedSize {
        len: usize,
        width: usize,
    },
}

#[derive(Clone)]
pub struct Grid<T: Default> {
    data: HashMap<Position, T>,
    len: usize,
    width: usize,
}

impl<T: Debug + Default> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|(k, v)| format!("{k:?}: {v:?}"))
                .join("\n"),
        )
    }
}

impl<T: Default> Index<Position> for Grid<T> {
    type Output = T;
    fn index(&self, _: Position) -> &Self::Output {
        unimplemented!()
    }
}

impl<T: Default> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.data.entry(index).or_default()
    }
}

impl<T: Default> Index<Cursor<T>> for Grid<T> {
    type Output = T;
    fn index(&self, cursor: Cursor<T>) -> &Self::Output {
        &self[cursor.index]
    }
}

impl<T: Default> Index<CursorMut<T>> for Grid<T> {
    type Output = T;
    fn index(&self, cursor: CursorMut<T>) -> &Self::Output {
        &self[cursor.index]
    }
}

impl<T: Default> IndexMut<CursorMut<T>> for Grid<T> {
    fn index_mut(&mut self, cursor: CursorMut<T>) -> &mut Self::Output {
        &mut self[cursor.index]
    }
}

impl<T: Default> Grid<T> {
    pub fn new(boundary: GridBoundary) -> Self {
        let (len, width) = match boundary {
            GridBoundary::Endless => (usize::MAX, usize::MAX),
            GridBoundary::PredefinedSize { len, width } => (len, width),
        };
        Self {
            data: HashMap::new(),
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

    pub fn cursor(&self, index: Position) -> Cursor<T> {
        Cursor::new(index, self)
    }

    pub fn cursor_mut(&mut self, index: Position) -> CursorMut<T> {
        CursorMut::new(index, self)
    }

    // pub fn iter(&self) -> impl Iterator<Item = Cursor<T>> + use<'_, T> {
    //     self.data.keys().map(|pos| self.cursor(*pos))
    // }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = CursorMut<T>> + use<'_, T> {
        self.data
            .keys()
            .cloned()
            .collect_vec()
            .into_iter()
            .map(|pos| self.cursor_mut(pos))
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

    pub fn invert(self) -> Self {
        Self {
            data: self
                .data
                .into_iter()
                .map(|(pos, val)| (Position::new(pos.y, pos.x), val))
                .collect(),
            len: self.width,
            width: self.len,
        }
    }
}

pub struct Cursor<T: Default> {
    grid: *const Grid<T>,
    pub index: Position,
}

pub struct CursorMut<T: Default> {
    grid: *mut Grid<T>,
    pub index: Position,
}

impl<T: Default> Copy for Cursor<T> {}
impl<T: Default> Copy for CursorMut<T> {}
impl<T: Default> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Default> Clone for CursorMut<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Default> Cursor<T> {
    fn new(index: Position, grid: *const Grid<T>) -> Self {
        Self { grid, index }
    }
}

impl<T: Default> Pointer<T> for Cursor<T> {
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

impl<T: Default> CursorMut<T> {
    fn new(index: Position, grid: *mut Grid<T>) -> Self {
        Self { grid, index }
    }

    fn len(&self) -> usize {
        unsafe { (*self.grid).len() }
    }

    fn width(&self) -> usize {
        unsafe { (*self.grid).width() }
    }
}

impl<T: Default> Pointer<T> for CursorMut<T> {
    fn index(&self) -> Position {
        self.index
    }

    fn reset(&mut self) -> &Self {
        self.index = Position::new(0, 0);
        self
    }

    fn val(&self) -> &T {
        unimplemented!()
    }

    fn to_enumerated_tuple(&self) -> (Position, T)
    where
        T: Clone,
    {
        (self.index, self.val().clone())
    }
}

impl<T: Default> PointerMut<T> for CursorMut<T> {
    fn val_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.grid)[self.index] }
    }
}

impl<T: Default> Directional for Cursor<T> {
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

impl<T: Default> Directional for CursorMut<T> {
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

impl<T: Default> Debug for Cursor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.index.fmt(f)
    }
}

impl<T: Default> Debug for CursorMut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.index.fmt(f)
    }
}

impl<T: Default> From<CursorMut<T>> for Cursor<T> {
    fn from(from: CursorMut<T>) -> Self {
        Self {
            grid: from.grid,
            index: from.index,
        }
    }
}

impl<T: Default> Add<SignedPosition> for CursorMut<T> {
    type Output = Self;
    fn add(mut self, rhs: SignedPosition) -> Self::Output {
        let new_pos = rhs + self.index;
        // println!("{} {}", self.len() as isize, self.width() as isize);
        // println!("new_pos: {new_pos:?}; %= ({}, {})", new_pos.x.rem_euclid(self.len() as isize), new_pos.y.rem_euclid(self.width() as isize));
        self.index.x = (new_pos.x.rem_euclid(self.len() as isize)) as usize;
        self.index.y = (new_pos.y.rem_euclid(self.width() as isize)) as usize;
        self
    }
}

impl<T: Default + ToString> Grid<T> {
    pub fn display(&mut self) -> String {
        (0..self.len)
            .into_iter()
            .map(move |x| {
                (0..self.width)
                    .into_iter()
                    .map(|y| self.cursor_mut(Position::new(x, y)).val_mut().to_string())
                    .join("")
            })
            .join("\n")
    }
}
