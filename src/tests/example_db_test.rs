#[cfg(test)]
mod tests {
    use diesel::{QueryableByName, RunQueryDsl, sql_query};
    use diesel::sql_types::Integer;
    use crate::tests::common_setup::common::get_common_testing_context;

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
}
