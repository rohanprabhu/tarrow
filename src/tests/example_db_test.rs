#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use diesel::{PgConnection, QueryableByName, RunQueryDsl, sql_query};
    use diesel::sql_types::Integer;
    use serde_json::{Map, Value};
    use crate::object_db::models::StoreObjectRequest;
    use crate::object_db::object_db::ObjectDb;
    use crate::tests::common_setup::common::{get_common_testing_context, init_test, TestingContext};

    trait UnwrapPgConn<'a> {
        fn get_pg_conn(&self) -> &'a mut PgConnection;
    }

    impl <'a> UnwrapPgConn<'a> for &Mutex<TestingContext> {
        fn get_pg_conn(&self) -> &'a mut PgConnection {
            todo!();
            //self.lock().unwrap().pg_conn()
        }
    }

    #[tokio::test]
    async fn test_db_should_connect() {
        #[derive(QueryableByName, Debug)]
        struct ExampleDbTestData {
            #[sql_type = "Integer"]
            pub result: i32
        }

        let common_context = get_common_testing_context()
            .await;

        let result : Vec<ExampleDbTestData> = sql_query(r#"
            SELECT 1 + 1 as result
        "#)
            .load(common_context.lock().unwrap().pg_conn())
            .unwrap();

        println!("Result {:?}", result);
    }

    #[tokio::test]
    async fn insert_blob() {
        init_test();
        let common_context = get_common_testing_context()
            .await;

        let object_db = ObjectDb {};
        let result = object_db.store_object(
            common_context.lock().unwrap().pg_conn(),
            &StoreObjectRequest {
                object_data: String::from("Hello World!!").into_bytes(),
                metadata: Value::Object(Map::new())
            }
        ).unwrap();
    }
}
