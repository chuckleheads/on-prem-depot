CREATE TABLE IF NOT EXISTS origin_invitations (
  id PRIMARY KEY SEQUENCE,
  origin_id int REFERENCES origins(id),
  origin_name text,
  account_id bigint,
  account_name text,
  owner_id int,
  ignored bool DEFAULT false,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  account_sync bool DEFAULT false,
  UNIQUE (origin_id, account_id)
);