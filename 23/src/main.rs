#![feature(iter_array_chunks)]
#![feature(slice_group_by)]

use aoc_proc::{get_runner, get_all_runs};
use itertools::Itertools;
use util::NulBoxError;
use util::aoc::*;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

fn main() -> NulBoxError {
    println!("AoC 2023 Results:");
    get_all_runs!().iter().group_by(|run| run.day).into_iter()
    .map(|(day, runs)| -> NulBoxError {
        println!("   -----{day}-----");
        runs.sorted().map(|run| -> NulBoxError {
            let part = run.part;
            let (runner, input_file) = get_runner!(run);
            println!(
                "    Part {part}: {}",
                runner.solve(input_file)?
            );
            Ok(())
        }).collect::<Result<Vec<()>, BoxError>>()?;
        Ok(())
    }).collect::<Result<Vec<()>, BoxError>>()?;

    Ok(())
}
