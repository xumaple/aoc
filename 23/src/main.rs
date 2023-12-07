#![feature(iter_array_chunks)]

use util::NulBoxError;
use util::aoc::*;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

fn run_day(day: Day) {
    match Day {
        Day::D01 => d01,
        Day::D02 => d02,
        Day::D03 => d03,
        Day::D04 => d04,
        Day::D05 => d05,
        Day::D06 => d06,
        Day::D07 => d07,
        _ => unimplemented!()
    }::a::run("src/d01/input.txt");
}

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
