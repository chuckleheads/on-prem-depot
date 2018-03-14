CREATE TABLE IF NOT EXISTS origin_secret_keys (
  id PRIMARY KEY SEQUENCE,
  origin_id int REFERENCES origins(id),
  owner_id int,
  name text,
  revision text,
  full_name text UNIQUE,
  body bytea,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);