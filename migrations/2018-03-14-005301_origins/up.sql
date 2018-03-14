CREATE TABLE IF NOT EXISTS origins (
  id PRIMARY KEY SEQUENCE,
  name text UNIQUE,
  owner_id int,
  session_sync bool DEFAULT false,
  default_package_visibility text NOT NULL DEFAULT 'public'
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
);
