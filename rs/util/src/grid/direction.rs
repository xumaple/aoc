use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::{UnsafeFrom, UnsafeInto};

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