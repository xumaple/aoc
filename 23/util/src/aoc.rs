pub mod run;
pub use super::{error::*, Path};
pub use run::*;

pub use clap::Parser;
pub use std::fmt::Debug;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Module to run in format <DDP>, eg. `01a`
    pub module: Option<String>,

    #[arg(short, long)]
    pub filename: Option<String>,
}

// !!! Warning: If changing this trait, make sure to also change its
// default implementation in aoc-proc/lib.rs
pub trait Runner: Debug {
    fn solve(&self, filename: impl AsRef<Path>) -> Result<u64, BoxError>;
}
