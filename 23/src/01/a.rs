use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = 0;
    for l in read_lines("src/01/input.txt")? {
        let l = l?;
        let c1 = l.chars().find(|c| c.is_digit(10));
        let c2 = l.chars().rfind(|c| c.is_digit(10));
        sum += 10 * c1.unwrap().to_digit(10).unwrap() + c2.unwrap().to_digit(10).unwrap();
    }

    println!("{sum}");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
