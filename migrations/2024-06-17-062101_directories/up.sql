CREATE TABLE directories (
    id CHAR(32) PRIMARY KEY NOT NULL,
    bucket_id CHAR(32) NOT NULL,
    name VARCHAR(50) NOT NULL,
    label VARCHAR(100) NOT NULL,
    file_count INTEGER NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    FOREIGN KEY (bucket_id) REFERENCES buckets(id)
);
CREATE INDEX directories_bucket_id_idx ON directories(bucket_id);
CREATE UNIQUE INDEX directories_bucket_id_name_idx ON directories(bucket_id, name);
CREATE INDEX directories_bucket_id_label_idx ON directories(bucket_id, label);
