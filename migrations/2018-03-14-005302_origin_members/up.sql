CREATE TABLE IF NOT EXISTS origin_members (
  origin_id bigint REFERENCES origins(id),
  origin_name text,
  account_id bigint,
  account_name text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
  PRIMARY KEY (origin_id, account_id)
);