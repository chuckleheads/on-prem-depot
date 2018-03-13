
CREATE SEQUENCE IF NOT EXISTS origin_id_seq;

CREATE TABLE IF NOT EXISTS origins (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
  name text UNIQUE,
  owner_id bigint,
  session_sync bool DEFAULT false,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);

CREATE TABLE IF NOT EXISTS origin_members (
  origin_id bigint REFERENCES origins(id),
  origin_name text,
  account_id bigint,
  account_name text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  PRIMARY KEY (origin_id, account_id)
);