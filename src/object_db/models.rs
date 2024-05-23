pub struct StoreObjectRequest {
    pub object_data: Vec<u8>,
    pub metadata: serde_json::Value
}

pub struct StoredObject {
    pub content_address_sha256: [u8; 32],
}
