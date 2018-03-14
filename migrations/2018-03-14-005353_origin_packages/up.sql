CREATE TABLE IF NOT EXISTS origin_packages (
  id PRIMARY KEY SEQUENCE,
  origin_id int REFERENCES origins(id),
  owner_id int,
  name text,
  ident text UNIQUE,
  checksum text,
  manifest text,
  config text,
  target text,
  deps text,
  tdeps text,
  exposes text,
  scheduler_sync bool DEFAULT false,
  visibility text NOT NULL DEFAULT 'public',
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);