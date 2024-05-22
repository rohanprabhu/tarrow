CREATE TABLE blobs (
    id BIGSERIAL PRIMARY KEY,
    content_address_sha256 BYTEA UNIQUE,
    content BYTEA NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    CHECK ( length(content_address_sha256) = 32)
);
