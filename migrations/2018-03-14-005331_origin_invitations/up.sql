CREATE SEQUENCE IF NOT EXISTS origin_invitations_id_seq;

CREATE TABLE IF NOT EXISTS origin_invitations (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_invitations_id_seq'),
  origin_id bigint REFERENCES origins(id),
  origin_name text,
  account_id bigint,
  account_name text,
  owner_id bigint,
  ignored bool DEFAULT false,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  account_sync bool DEFAULT false,
  UNIQUE (origin_id, account_id)
);