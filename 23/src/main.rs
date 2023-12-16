#![feature(iter_array_chunks)]
#![feature(slice_group_by)]

use aoc_proc::{get_all_runs, run};
use util::aoc::*;
use util::Itertools;
use util::NulBoxError;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
// mod d12;
mod d13;
mod d14;
mod d15;
mod d16;

fn main() -> NulBoxError {
    let args = CliArgs::parse();
    if let Some(r) = args.module.as_deref() {
        let runner = r.parse::<Run>()?;
        println!(
            "{}",
            run!(runner, args.filename.unwrap_or("input.txt".to_string()))?
        );
        return Ok(());
    }
    if args.filename.is_some() {
        return Err(E::CommandLineError(
            "Must specify module in format <DDP>, eg. `cargo run -- 01a`",
        ))?;
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
                    println!("    Part {}: {}", runner.part, run!(runner, "input.txt")?);
                    Ok(())
                })
                .collect::<Result<Vec<()>, BoxError>>()?;
            Ok(())
        })
        .collect::<Result<Vec<()>, BoxError>>()?;

    Ok(())
}
