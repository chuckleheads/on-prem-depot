CREATE TABLE IF NOT EXISTS origin_public_keys (
  id bigserial PRIMARY KEY,
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  revision text,
  full_name text UNIQUE,
  body bytea,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);
