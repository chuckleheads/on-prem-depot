CREATE TABLE IF NOT EXISTS origin_packages (
  id bigserial PRIMARY KEY,
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  ident text UNIQUE,
  checksum text,
  manifest text,
  config text,
  target text,
  deps text,
  tdeps text,
  exposes text,
  visibility text NOT NULL DEFAULT 'public',
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);
