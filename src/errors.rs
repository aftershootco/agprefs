#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Failed to parse")]
    Nom(String),
    #[error("{0}")]
    Other(String),
    #[error("{0}")]
    Compose(#[from] cookie_factory::GenError),
}

impl From<nom::Err<nom::error::Error<&str>>> for Errors {
    fn from(e: nom::Err<nom::error::Error<&str>>) -> Self {
        Errors::Nom(e.to_string())
    }
}
