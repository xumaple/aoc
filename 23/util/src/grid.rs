use std::slice::SliceIndex;

use crate::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Copy)]
pub struct Position<'a, T> {
    pub x: usize,
    pub y: usize,
    grid: Option<&'a Grid<T>>,
}

impl<'a, T> Debug for Position<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Direction {
    L = 0,
    D = 1,
    R = 2,
    U = 3,
}

impl Direction {
    #[allow(non_snake_case)]
    pub fn turn_L(&self) -> Self {
        self.shift(1)
    }

    #[allow(non_snake_case)]
    pub fn turn_R(&self) -> Self {
        self.shift(3)
    }

    pub fn turn_back(&self) -> Self {
        self.shift(2)
    }

    fn shift(&self, n: u8) -> Self {
        ((*self as u8 + n) % 4).uinto()
    }
}

impl UnsafeFrom<u8> for Direction {
    fn ufrom(a: u8) -> Self {
        Self::from_u8(a).unwrap()
    }
}

impl<'a, T> Position<'a, T> {
    pub fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn next(&self, dir: Direction) -> Option<Position<T>> {
        if match dir {
            Direction::U => self.x <= 0,
            Direction::D => {
                if let Some(grid) = self.grid {
                    self.x >= grid.len() - 1
                } else {
                    false
                }
            }
            Direction::L => self.y <= 0,
            Direction::R => {
                if let Some(grid) = self.grid {
                    self.y >= grid.width() - 1
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

    pub fn new(x: usize, y: usize, grid: Option<&'a Grid<T>>) -> Self {
        Self::assert_within_range(x, y, grid);
        Self { x, y, grid }
    }

    pub fn new_from_tuple(coords: (usize, usize), grid: Option<&'a Grid<T>>) -> Self {
        Self::new(coords.0, coords.1, grid)
    }

    pub fn refresh_lifetime<'b>(self, new_grid: &'b Grid<T>) -> Position<'b, T> {
        Position::new(self.x, self.y, Some(new_grid))
    }

    fn assert_within_range(x: usize, y: usize, grid: Option<&'a Grid<T>>) {
        if let Some(grid) = grid {
            if x < grid.len() && y < grid.width() {
                return;
            }
            println!("x: {x} <? {}; y: {y} <? {}", grid.len(), grid.width());
            assert!(x < grid.len() && y < grid.width());
        }
    }

    pub fn pos<O>(&self) -> Position<'static, O> {
        Position::new(self.x, self.y, None)
    }
}

impl<'a, T> Clone for Position<'a, T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            grid: self.grid,
        }
    }
}

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

    pub fn iter_rows<'a>(&'a self) -> std::slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn iter_rows_mut<'a>(&'a mut self) -> std::slice::IterMut<Vec<T>> {
        self.0.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flatten()
    }
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

impl<'a, T> Index<Position<'a, T>> for Grid<T> {
    type Output = T;
    fn index(&self, index: Position<'a, T>) -> &Self::Output {
        &self[index.x][index.y]
    }
}

impl<'a, T> IndexMut<Position<'a, T>> for Grid<T> {
    fn index_mut(&mut self, index: Position<'a, T>) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}
