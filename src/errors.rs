#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Failed to parse")]
    Nom,
    #[error("{0}")]
    Other(String),
}

impl From<nom::Err<nom::error::Error<&'static str>>> for Errors {
    fn from(_: nom::Err<nom::error::Error<&'static str>>) -> Self {
        Errors::Nom
    }
}
