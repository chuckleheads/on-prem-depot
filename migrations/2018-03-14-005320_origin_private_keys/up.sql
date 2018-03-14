CREATE SEQUENCE IF NOT EXISTS origin_secret_key_id_seq;

CREATE TABLE IF NOT EXISTS origin_secret_keys (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_secret_key_id_seq'),
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  revision text,
  full_name text UNIQUE,
  body bytea,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);