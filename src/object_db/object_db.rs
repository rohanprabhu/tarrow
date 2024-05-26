use std::cmp::{max, min};
use diesel::{insert_into, PgConnection, QueryResult, RunQueryDsl};
use diesel::result::{DatabaseErrorKind, Error};
use sha2::{Digest, Sha256};
use log::debug;
use crate::core::types::ContentAddress;
use crate::object_db::errors::ObjectStorageError::ObjectCollisionError;
use crate::object_db::models::{StoredObject, StoreObjectRequest};
use crate::schema::blobs::dsl::blobs;

mod record_models {
    use diesel::{Insertable, Queryable};
    use crate::core::types::ContentAddressRaw;
    use crate::schema::blobs;

    #[derive(Insertable, Queryable)]
    #[diesel(table_name = blobs)]
    pub struct BlobRecord<'a> {
        pub content_address_sha256: &'a ContentAddressRaw,
        pub content: &'a ContentAddressRaw
    }
}

pub struct ObjectDb {
}

fn sha_256(content: &Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let mut sha256 = Sha256::new();
    sha256.update(content);
    Ok(sha256.finalize().to_vec())
}

impl ObjectDb {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_blobs(
        self: &Self,
        content_addresses: Vec<ContentAddress>
    ) -> Result<Vec<StoredObject>, anyhow::Error> {
        todo!()
    }

    pub fn store_object(
        self: &Self,
        pg_connection: &mut PgConnection,
        store_object: &StoreObjectRequest
    ) -> Result<StoredObject, anyhow::Error> {
        let content_address_sha256 = &sha_256(&store_object.object_data)?;
        debug!(
            "Generated content hash {:?} for {:?} (truncated)",
            content_address_sha256,
            &(store_object.object_data[0 .. min(50, store_object.object_data.len() - 1)])
        );

        let db_store_result = insert_into(blobs)
            .values(record_models::BlobRecord {
                content: &(store_object.object_data),
                content_address_sha256
            })
            .execute(pg_connection);

        match db_store_result {
            Ok(_) => {
                Ok(StoredObject {
                    content_address_sha256: content_address_sha256.as_slice().try_into()?
                })
            }

            // TODO - Is there a better way to write multi-match statements?
            Err(err) => {
                // EDU - Removing the '&' fails - seems like a very common error with the borrow
                // checker. While it is obvious that we are borrowing instead of a move in the match
                // arms, but the error specifically shows up only when we use error_info - if
                // we somehow skip it, then even without borrowing `err`, a partial move does not
                // occur - which is fairly curious...
                match &err {
                    Error::DatabaseError(database_error_kind, error_info) => {
                        match database_error_kind {
                            DatabaseErrorKind::UniqueViolation => {
                                // TODO - Check if the constraint name is correct
                                Err(anyhow::Error::new(ObjectCollisionError {
                                    conflicting_content_sha256: content_address_sha256.as_slice()
                                        .try_into()?
                                }))
                            }
                            _ => Err(anyhow::Error::new(err))
                        }
                    }
                    _ => Err(anyhow::Error::new(err))
                }
            }
        }
    }
}
