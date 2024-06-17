CREATE TABLE buckets (
  id CHAR(32) PRIMARY KEY,
  client_id CHAR(32) NOT NULL,
  name VARCHAR NOT NULL,
  label VARCHAR NOT NULL
);
CREATE INDEX buckets_client_id_idx ON buckets(client_id);
