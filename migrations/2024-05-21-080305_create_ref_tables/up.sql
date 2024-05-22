CREATE TABLE refs(
    id BIGSERIAL PRIMARY KEY,
    ref_name CHARACTER VARYING,
    tree_id BIGINT NOT NULL REFERENCES trees(id),
    parent_ref BIGINT REFERENCES refs(id),
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB
);

CREATE UNIQUE INDEX idx_only_one_head_ref
    ON refs (ref_name, (parent_ref IS NULL))
WHERE refs.parent_ref IS NULL;
