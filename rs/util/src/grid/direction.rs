use crate::{PartialEq, UnsafeFrom, UnsafeInto};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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

    pub fn iter_counterclockwise() -> impl Iterator<Item = Self> {
        (0..4).map(Self::ufrom)
    }

    pub fn iter_clockwise() -> impl Iterator<Item = Self> {
        [0, 3, 2, 1].into_iter().map(Self::ufrom)
    }
}

impl UnsafeFrom<u8> for Direction {
    fn ufrom(a: u8) -> Self {
        Self::from_u8(a).unwrap()
    }
}

pub trait Directional: Sized {
    type Err: Default;
    fn next(&self, dir: Direction) -> Option<Self>;
    fn step(&mut self, dir: Direction) -> Result<&Self, Self::Err> {
        match self.next(dir) {
            Some(new_self) => {
                *self = new_self;
                Ok(self)
            }
            None => Err(self.error(dir)),
        }
    }
    fn error(&self, _dir: Direction) -> Self::Err {
        Self::Err::default()
    }

    fn check_sides<F, R>(&self, mut f: F) -> impl Iterator<Item = R>
    where
        F: FnMut(Option<Self>) -> R,
    {
        Direction::iter_clockwise().map(move |dir| f(self.next(dir)))
    }

    // fn check_diagonals<F, R>(&self, mut f: F) -> impl Iterator<Item = Option<R>>
    // where
    //     F: FnMut(Self) -> R,
    // {}

    fn exists_and_matches<F>(&self, dir: Direction, matches: F) -> bool
    where
        Self: Copy,
        F: Fn(Self, Self) -> bool,
    {
        self.next(dir)
            .is_some_and(|new_self| matches(*self, new_self))
    }
}
