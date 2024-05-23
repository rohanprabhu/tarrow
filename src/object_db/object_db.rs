use std::cmp::{max, min};
use diesel::{insert_into, PgConnection, RunQueryDsl};
use sha2::{Digest, Sha256};
use log::debug;
use crate::object_db::models::{StoredObject, StoreObjectRequest};
use crate::schema::blobs::dsl::blobs;

mod record_models {
    use diesel::{Insertable, Queryable};
    use crate::schema::blobs;

    #[derive(Insertable, Queryable)]
    #[diesel(table_name = blobs)]
    pub struct BlobRecord<'a> {
        pub content_address_sha256: &'a Vec<u8>,
        pub content: &'a Vec<u8>,
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
    fn new() -> Self {
        Self {}
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

        insert_into(blobs)
            .values(record_models::BlobRecord {
                content: &(store_object.object_data),
                content_address_sha256
            })
            .execute(pg_connection)?;


        Ok(StoredObject {
            content_address_sha256: content_address_sha256.as_slice().try_into()?
        })
    }
}
