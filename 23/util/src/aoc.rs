pub mod run;
pub use run::*;
pub use super::error::*;

pub use std::fmt::Debug;

pub trait Runner: Debug  {
    fn solve(&self, filename: &str) -> Result<u64, BoxError>;
}
