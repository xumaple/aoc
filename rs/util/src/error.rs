use super::{Direction, Position};

pub type BoxError = Box<dyn std::error::Error>;
pub type NulBoxError = Result<(), BoxError>;

#[derive(thiserror::Error, Debug, Default)]
pub enum E {
    #[default]
    #[error("Generic error")]
    Error,
    #[error("Unable to find split {0} in string")]
    SplitError(String),
    #[error("Unable to parse from string")]
    ParseError,
    #[error("Unable to parse command line: {0}")]
    CommandLineError(&'static str),
    #[error("Algorithm error: {0}")]
    AlgorithmError(&'static str),
    #[error("Out of bounds at: {0:?} going {1:?}")]
    OutOfBoundsMove(Position, Direction),
    #[error("Out of bounds at: {0:?}")]
    OutOfBounds(Position),
}
