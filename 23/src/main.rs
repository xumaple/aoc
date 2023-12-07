#![feature(iter_array_chunks)]

use util::NulBoxError;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

fn main() -> NulBoxError {
    println!("AoC 2023 Results:");
    println!(
        "  Day 1 -  A: {} ; B: {}",
        d01::a::run("src/d01/input.txt")?,
        d01::b::run("src/d01/input.txt")?
    );
    println!(
        "  Day 2 -  A: {} ; B: {}",
        d02::a::run("src/d02/input.txt")?,
        d02::b::run("src/d02/input.txt")?
    );
    println!(
        "  Day 3 -  A: {} ; B: {}",
        d03::a::run("src/d03/input.txt")?,
        d03::b::run("src/d03/input.txt")?
    );
    println!(
        "  Day 4 -  A: {} ; B: {}",
        d04::a::run("src/d04/input.txt")?,
        d04::b::run("src/d04/input.txt")?
    );
    println!(
        "  Day 5 -  A: {} ; B: {}",
        d05::a::run("src/d05/input.txt")?,
        d05::b::run("src/d05/input.txt")?
    );
    println!(
        "  Day 6 -  A: {} ; B: {}",
        d06::a::run("src/d06/input.txt")?,
        d06::b::run("src/d06/input.txt")?
    );
    println!(
        "  Day 7 -  A: {} ; B: {}",
        d07::a::run("src/d07/input.txt")?,
        d07::b::run("src/d07/input.txt")?
    );
    println!(
        "  Day 8 -  A: {} ; B: {}",
        d07::a::run("src/d08/input.txt")?,
        d07::b::run("src/d08/input.txt")?
    );

    Ok(())
}
