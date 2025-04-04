use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("RegexError: {0}")]
    RegexError(#[from] regex::Error),
    #[error("NotFound: {0}")]
    NotFound(String),
    #[error("unknown error: {0}")]
    Unknown(String),
}
