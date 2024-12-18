#![feature(pattern)]

use std::any::type_name;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub use core::str::FromStr;
pub use itertools::Itertools;
pub use multi_key_map::MultiKeyMap;
pub use num::integer::lcm;
pub use pathfinding;
pub use rc_cell::*;
pub use regex::Regex;
pub use std::cell::{Ref, RefMut};
pub use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
pub use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
};
pub use std::fmt::Debug;
pub use std::hash::Hash;
pub use std::iter::FromIterator;
pub use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Sub,
    SubAssign,
};
pub use std::path::Path;
pub use std::str::pattern::Pattern;
pub use std::sync::Arc;

pub mod algorithm;
pub mod aoc;
pub use algorithm::*;
pub mod error;
pub use error::*;
pub mod grid;
pub use grid::*;
pub mod string;
pub use string::*;
pub mod math;
pub use math::*;
pub mod multimap;
pub use multimap::*;

pub fn read<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path)
}

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn debug<T: Debug>(input: T) -> T {
    println!("{:?}", input);
    input
}

pub trait UnsafeFrom<T>: Sized {
    fn ufrom(input: T) -> Self;
}

macro_rules! impl_unsafe_from {
    ($t:ty) => {
        impl<S> UnsafeFrom<$t> for S
        where
            S: FromStr,
        {
            fn ufrom(input: $t) -> S {
                match input.parse::<S>() {
                    Ok(v) => v,
                    Err(_) => panic!(
                        "Unable to parse value: {} from type: {} to type: {}",
                        input,
                        type_name::<$t>(),
                        type_name::<S>(),
                    ),
                }
            }
        }
    };
}

impl_unsafe_from!(String);
impl_unsafe_from!(&str);

impl<A> UnsafeFrom<char> for A
where
    A: Number + TryFrom<char> + Sub<A, Output = A>,
{
    fn ufrom(input: char) -> Self {
        match A::try_from(input) {
            Ok(v) => v - A::from(0x30).unwrap(),
            Err(_) => panic!(
                "Unable to parse value: {} from type: {} to type: {}",
                input,
                type_name::<char>(),
                type_name::<A>(),
            ),
        }
    }
}

impl UnsafeFrom<char> for char {
    fn ufrom(input: char) -> Self {
        input
    }
}

pub trait UnsafeInto<S>: Sized {
    fn uinto(self) -> S;
}

impl<T, S> UnsafeInto<S> for T
where
    S: UnsafeFrom<T>,
{
    fn uinto(self) -> S {
        S::ufrom(self)
    }
}

pub trait IteratorExt: Iterator {
    fn unext(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

impl<I, It: Iterator<Item = I>> IteratorExt for It {}
