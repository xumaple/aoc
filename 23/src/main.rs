use util::NulBoxError;

mod d01;
mod d02;
mod d03;
mod d04;

fn main() -> NulBoxError {
    println!("AoC 2023 Results:");
    println!("  Day 1 -  A: {}; B: {}", d01::a::run("src/d01/input.txt")?, d01::b::run("src/d01/input.txt")?);
    println!("  Day 2 -  A: {}; B: {}", d02::a::run("src/d02/input.txt")?, d02::b::run("src/d02/input.txt")?);
    println!("  Day 3 -  A: {}; B: {}", d03::a::run("src/d03/input.txt")?, d03::b::run("src/d03/input.txt")?);
    // println!("  Day 4 -  A: {}; B: {}", d04::a::run("src/d04/input.txt")?, d04::b::run("src/d04/input.txt")?);

    Ok(())
}