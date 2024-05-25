use std::fmt::{Display, Formatter, write};
use derive_more::Error;

#[derive(Error, Debug)]
pub enum ObjectStorageError {
    ObjectCollisionError {
        conflicting_content_sha256: [u8; 32]
    }
}

impl Display for ObjectStorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectStorageError::ObjectCollisionError { conflicting_content_sha256 } => {
                // TODO - This should ideally show the hex values for the conflicting digest
                write!(
                    f, "ObjectStorageError::ObjectCollisionError[conflicting_digest={:?}]",
                    conflicting_content_sha256
                )
            }
        }
    }
}
