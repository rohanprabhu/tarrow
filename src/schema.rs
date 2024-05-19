// @generated automatically by Diesel CLI.

diesel::table! {
    tarrow_leaf_objects (id) {
        id -> Int8,
        content_address_sha256 -> Nullable<Bytea>,
        content -> Bytea,
        metadata -> Jsonb,
    }
}
