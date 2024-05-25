#[cfg(test)]
mod tests {
    use diesel::{QueryableByName, RunQueryDsl, sql_query};
    use diesel::sql_types::Integer;
    use log::debug;
    use serde_json::{Map, Value};
    use crate::object_db::errors::ObjectStorageError;
    use crate::object_db::errors::ObjectStorageError::ObjectCollisionError;
    use crate::object_db::models::StoreObjectRequest;
    use crate::object_db::object_db::ObjectDb;
    use crate::tests::common_setup::common::{get_common_testing_context, init_test, TestingContext};

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

        let object_db = ObjectDb::new();
        let result = object_db.store_object(
            common_context.lock().unwrap().pg_conn(),
            &StoreObjectRequest {
                object_data: String::from("Hello World!!").into_bytes(),
                metadata: Value::Object(Map::new())
            }
        ).unwrap();
    }

    #[tokio::test]
    async fn object_collision_test() {
        init_test();
        let common_context = get_common_testing_context()
            .await;

        let object_db = ObjectDb::new();
        let store_base_data = object_db.store_object(
            common_context.lock().unwrap().pg_conn(),
            &StoreObjectRequest {
                object_data: String::from("Object Collision Test").into_bytes(),
                metadata: Value::Object(Map::new())
            }
        ).unwrap();

        // This should cause an error, specifically an `ObjectCollisionError`
        let store_duplicate_data = object_db.store_object(
            common_context.lock().unwrap().pg_conn(),
            &StoreObjectRequest {
                object_data: String::from("Object Collision Test").into_bytes(),
                metadata: Value::Object(Map::new())
            }
        );

        assert!(store_duplicate_data.is_err());

        let reported_error = store_duplicate_data.err()
            .unwrap();

        match reported_error.downcast_ref::<ObjectStorageError>() {
            Some(ObjectCollisionError { conflicting_content_sha256}) => {
                debug!("Valid and expected error type");
                assert_eq!(conflicting_content_sha256, &store_base_data.content_address_sha256);
            }

            _ => {
                assert!(false, "The returned error must be of type ObjectCollisionError");
            }
        }
    }
}
