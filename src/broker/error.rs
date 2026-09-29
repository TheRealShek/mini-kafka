use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrokerError {
    #[error("offset doesn't exist")]
    OffsetOutOfRange,
}
