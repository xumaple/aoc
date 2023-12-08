#![feature(iter_array_chunks)]
#![feature(slice_group_by)]

use aoc_proc::{get_all_runs, run};
use itertools::Itertools;
use std::env;
use util::aoc::*;
use util::NulBoxError;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;

fn main() -> NulBoxError {
    let mut args = env::args();
    let _ = args.next();
    if let Some(s) = args.next() {
        let runner = s.parse::<Run>()?;
        if let Some(_) = args.next() {
            return Err(E::CommandLineError("Too many arguments"))?;
        }
        println!("{}", run!(runner)?);
        return Ok(());
    }

    println!("AoC 2023 Results:");
    get_all_runs!()
        .iter()
        .group_by(|run| run.day)
        .into_iter()
        .map(|(day, runs)| -> NulBoxError {
            println!("   -----{day}-----");
            runs.sorted()
                .map(|runner| -> NulBoxError {
                    println!("    Part {}: {}", runner.part, run!(runner)?);
                    Ok(())
                })
                .collect::<Result<Vec<()>, BoxError>>()?;
            Ok(())
        })
        .collect::<Result<Vec<()>, BoxError>>()?;

    Ok(())
}
