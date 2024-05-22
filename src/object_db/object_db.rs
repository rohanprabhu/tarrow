use diesel::PgConnection;
use crate::object_db::models::{StoredObject, StoreObjectRequest};

pub struct ObjectDb {
}

impl ObjectDb {
    fn new() -> Self {
        Self {}
    }

    fn store_object(
        self: &mut Self,
        pg_connection: &PgConnection,
        store_object: StoreObjectRequest
    ) -> Result<StoredObject, anyhow::Error> {
        todo!()
    }
}
