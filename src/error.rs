#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bad magic value: {0}")]
    BadMagic(String),
    #[error("unknown property type: {0}")]
    UnknownPropertyType(String),
    #[error("unknown PropertyMeta type: {0}")]
    UnknownPropertyMeta(String),
    #[error("unknown vec type: {0}")]
    UnknownVecType(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

#[derive(thiserror::Error, Debug)]
#[error("at offset 0x{offset:x}: {error}")]
pub struct ParseError {
    pub offset: usize,
    pub error: Error,
}
