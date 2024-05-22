// @generated automatically by Diesel CLI.

diesel::table! {
    blobs (id) {
        id -> Int8,
        content_address_sha256 -> Nullable<Bytea>,
        content -> Bytea,
        metadata -> Jsonb,
    }
}

diesel::table! {
    refs (id) {
        id -> Int8,
        ref_name -> Nullable<Varchar>,
        tree_id -> Int8,
        parent_ref -> Nullable<Int8>,
        metadata -> Jsonb,
    }
}

diesel::table! {
    tree_entries (id) {
        id -> Int8,
        parent_id -> Nullable<Int8>,
        path -> Array<Nullable<Text>>,
        blob_id -> Nullable<Int8>,
        tree_id -> Nullable<Int8>,
    }
}

diesel::table! {
    trees (id) {
        id -> Int8,
        content_address_sha256 -> Bytea,
    }
}

diesel::joinable!(refs -> trees (tree_id));
diesel::joinable!(tree_entries -> blobs (blob_id));

diesel::allow_tables_to_appear_in_same_query!(
    blobs,
    refs,
    tree_entries,
    trees,
);
