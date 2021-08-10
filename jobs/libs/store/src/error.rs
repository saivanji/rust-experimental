use derive_more::Display;
use tokio_postgres::error::Error;

#[derive(Debug, Display)]
pub enum StoreError {
    InvalidConnAddress,
    PoolCreationFailure,
    ConnectionFailure,
    DatabaseError(String),
}

impl std::error::Error for StoreError {}

impl From<Error> for StoreError {
    fn from(err: Error) -> Self {
        let message = format!("{}", err);

        Self::DatabaseError(message)
    }
}

pub type Result<T> = std::result::Result<T, StoreError>;
