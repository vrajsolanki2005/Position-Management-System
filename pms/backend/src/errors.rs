use thiserror::Error;

#[derive(Debug, Error)]
pub enum SvcError {
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("DB error: {0}")]
    Db(String),
    #[error("Serialization error: {0}")]
    Serde(String),
    #[error("Invalid input: {0}")]
    Invalid(String),
}

pub type SvcResult<T> = Result<T, SvcError>;
