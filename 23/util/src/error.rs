#[derive(thiserror::Error, Debug)]
pub enum E {
    #[error("Unable to find split {0} in string")]
    SplitError(String),
    #[error("Unable to parse from string")]
    ParseError,
}
