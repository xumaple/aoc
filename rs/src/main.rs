#![feature(iter_array_chunks)]

use aoc_proc::{get_all_runs, run};
use util::aoc::*;
use util::Itertools;
use util::NulBoxError;

mod y23;

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
            "Must specify module in format <YY-DDP>, eg. `cargo run -- 23-01a`",
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
