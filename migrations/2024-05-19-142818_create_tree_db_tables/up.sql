CREATE TABLE trees (
    id BIGSERIAL PRIMARY KEY,
    content_address_sha256 BYTEA NOT NULL,
    CHECK ( length(content_address_sha256) = 32)
);

CREATE TABLE tree_entries (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT REFERENCES trees(id),
    path TEXT[] NOT NULL DEFAULT '{}'::TEXT[],
    blob_id BIGINT REFERENCES blobs(id),
    tree_id BIGINT REFERENCES trees(id),

    --- Must point to a tree or a blob
    CHECK (( tree_id IS NULL AND blob_id IS NOT NULL ) OR (blob_id IS NULL AND tree_id IS NOT NULL))
);
