pub mod day;
pub use day::*;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Part {
    A,
    B,
}

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Run {
    pub day: Day,
    pub part: Part,
}
