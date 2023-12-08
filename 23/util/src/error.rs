pub type BoxError = Box<dyn std::error::Error>;
pub type NulBoxError = Result<(), BoxError>;

#[derive(thiserror::Error, Debug)]
pub enum E {
    #[error("Unable to find split {0} in string")]
    SplitError(String),
    #[error("Unable to parse from string")]
    ParseError,
    #[error("Unable to parse command line: {0}")]
    CommandLineError(&'static str),
}
