CREATE TABLE IF NOT EXISTS origin_invitations (
  id bigserial PRIMARY KEY,
  origin_id bigint REFERENCES origins(id),
  origin_name text,
  account_id bigint,
  account_name text,
  owner_id bigint,
  ignored bool DEFAULT false,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  UNIQUE (origin_id, account_id)
);
