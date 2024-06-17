CREATE TABLE buckets (
    id CHAR(32) PRIMARY KEY,
    client_id CHAR(32) NOT NULL,
    name VARCHAR(50) NOT NULL,
    label VARCHAR(100) NOT NULL
);
CREATE INDEX buckets_client_id_idx ON buckets(client_id);
