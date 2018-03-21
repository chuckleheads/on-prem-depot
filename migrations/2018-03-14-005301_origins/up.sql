CREATE TABLE IF NOT EXISTS origins (
  id bigserial PRIMARY KEY,
  name text NOT NULL UNIQUE,
  owner_id bigint NOT NULL,
  default_package_visibility text NOT NULL DEFAULT 'public',
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now()
);
