use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub type BoxError = Box<dyn std::error::Error>;
pub type NulBoxError = Result<(), BoxError>;

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
