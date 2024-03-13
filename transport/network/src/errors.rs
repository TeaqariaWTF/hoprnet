use thiserror::Error;

/// Errors that can be generated by the crate.
#[derive(Error, Debug)]
pub enum NetworkingError {
    #[error("the network operation timed out after {0} seconds")]
    Timeout(u64),

    #[error("error in the messaging sub-protocol: {0}")]
    MessagingError(String),

    #[error("error while decoding message data")]
    DecodingError,

    #[error("error while performing an operation on own PeerId")]
    DisallowedOperationOnOwnPeerIdError,

    #[error("backend error: {0}")]
    BackendError(#[from] sqlx::Error),

    #[error("peer does not exist")]
    NonExistingPeer,

    #[error("{0}")]
    Other(String),
}

/// Result built on top of the crate error [NetworkingError]
pub type Result<T> = core::result::Result<T, NetworkingError>;