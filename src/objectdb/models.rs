use crate::schema::tarrow_leaf_objects::content_address_sha256;

pub struct StoreObjectRequest {
    object_data: Vec<u8>,
    metadata: serde_json::Value
}

pub struct StoredObject {
    content_address_sha256: [u8; 32],
}
