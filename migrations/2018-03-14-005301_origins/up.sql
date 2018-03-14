CREATE SEQUENCE IF NOT EXISTS origin_id_seq;

CREATE TABLE IF NOT EXISTS origins (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
  name text UNIQUE,
  owner_id bigint,
  session_sync bool DEFAULT false,
  default_package_visibility text NOT NULL DEFAULT 'public'
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
);
