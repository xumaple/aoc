#![feature(pattern)]

use std::any::type_name;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub mod error;
pub use error::*;
pub mod string;
pub use string::*;

pub type BoxError = Box<dyn std::error::Error>;
pub type NulBoxError = Result<(), BoxError>;

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub trait UnsafeFrom<T>: Sized {
    fn ufrom(input: T) -> Self;
}

impl<T, S> UnsafeFrom<T> for S
where
    S: std::str::FromStr,
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

pub trait Number: Debug + Clone {}
impl Number for isize {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for usize {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}

// Couldn't figure out a way to use UnsafeFrom, so temp solution is another trait.
// Goal is to remove this in future, if I can get around duplicate implementation issues
pub trait UnsafeFromNum<A>: Sized
where
    A: Number,
{
    fn ufromn(a: A) -> Self;
}

impl<A, B> UnsafeFromNum<A> for B
where
    A: Number,
    B: Number + TryFrom<A>,
{
    fn ufromn(a: A) -> B {
        match a.clone().try_into() {
            Ok(v) => v,
            Err(_) => panic!(
                "Unable to convert {:?} from {} to {}",
                a,
                type_name::<A>(),
                type_name::<B>()
            ),
        }
    }
}

pub trait UnsafeIntoNum<B>: Sized
where
    B: Number,
{
    fn uinton(self) -> B;
}

impl<B, A> UnsafeIntoNum<B> for A
where
    A: Number,
    B: Number + UnsafeFromNum<A>,
{
    fn uinton(self) -> B {
        B::ufromn(self)
    }
}
