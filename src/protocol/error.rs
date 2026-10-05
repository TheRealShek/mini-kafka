use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("invalid request type: {0}")]
    InvalidRequestType(u8),

    #[error("incomplete parsing")]
    IncompleteParse,

    #[error("invalid utf-8 string")]
    InvalidUtf8,

    #[error("cannot decode empty input")]
    EmptyInput,
}
