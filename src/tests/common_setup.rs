#[cfg(test)]
pub mod common {
    use std::sync::Mutex;
    use tokio::sync::OnceCell;

    use diesel::PgConnection;
    use diesel::prelude::*;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};
    use lazy_static::lazy_static;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    pub struct TestingContext {
        pg_connection: PgConnection,
    }

    impl TestingContext {
        pub fn pg_conn(self: &'_ mut Self) -> &'_ mut PgConnection {
            &mut self.pg_connection
        }
    }

    lazy_static! {
        pub static ref TEST_DB_PARAMS: OnceCell<Mutex<TestingContext>> = OnceCell::new();
    }

    pub async fn get_common_testing_context() -> &'static Mutex<TestingContext> {
        TEST_DB_PARAMS.get_or_init(|| async {
            let database_url = String::from(
                "postgres://tarrow-dev:tarrow-dev@localhost:35431/tarrow-test-db"
            );

            let pg_connection = PgConnection::establish(&database_url)
                .unwrap_or_else(|e| panic!("Error connecting to testing database (db_url={}), error={:?}",
                                           database_url, e));

            Mutex::new(TestingContext {
                pg_connection,
            })
        }).await
    }
}