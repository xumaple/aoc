#![feature(pattern)]

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

pub trait UnsafeParse<T> {
    fn uparse(input: &str) -> T;
}

impl<T> UnsafeParse<T> for T
where
    T: std::str::FromStr,
{
    fn uparse(input: &str) -> T {
        match input.parse::<T>() {
            Ok(v) => v,
            Err(_) => panic!("Unable to parse string value: {input}"),
        }
    }
}

