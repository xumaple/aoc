use std::fs::File;
use std::io::{self, BufReader, Lines, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for l in read_lines("src/01/input.txt")? {

    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}