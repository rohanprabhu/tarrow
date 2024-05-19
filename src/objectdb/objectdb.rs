use diesel::pg::PgConnection;
use crate::objectdb::models::{StoredObject, StoreObjectRequest};

struct ObjectDb {
    pg_connection: PgConnection
}

impl ObjectDb {
    fn new(pg_connection: PgConnection) -> Self {
        Self {
            pg_connection
        }
    }

    fn store_object(
        self: &mut Self,
        store_object: StoreObjectRequest
    ) -> Result<StoredObject, anyhow::Error> {
        todo!()
    }
}
