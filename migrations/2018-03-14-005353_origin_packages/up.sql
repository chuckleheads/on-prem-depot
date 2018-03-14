CREATE SEQUENCE IF NOT EXISTS origin_package_id_seq;

CREATE TABLE IF NOT EXISTS origin_packages (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_package_id_seq'),
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
  scheduler_sync bool DEFAULT false,
  visibility text NOT NULL DEFAULT 'public',
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);