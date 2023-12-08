#![feature(pattern)]

use std::any::type_name;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
pub use std::fmt::Debug;
pub use std::ops::Range;
pub use std::path::Path;
pub use std::str::FromStr;
pub use itertools::Itertools;
pub use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
pub use num::integer::lcm;

pub mod aoc;
pub mod error;
pub use error::*;
pub mod string;
pub use string::*;
pub mod math;
pub use math::*;

pub fn read<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path)
}

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub trait UnsafeFrom<T>: Sized {
    fn ufrom(input: T) -> Self;
}

impl<T, S> UnsafeFrom<T> for S
where
    S: FromStr,
    T: AsRef<str>,
{
    fn ufrom(input: T) -> S {
        match input.as_ref().parse::<S>() {
            Ok(v) => v,
            Err(_) => panic!(
                "Unable to parse value: {} from type: {} to type: {}",
                input.as_ref(),
                type_name::<T>(),
                type_name::<S>(),
            ),
        }
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
