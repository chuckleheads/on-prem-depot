CREATE TABLE IF NOT EXISTS origin_projects (
  id bigserial PRIMARY KEY,
  origin_id bigint REFERENCES origins(id),
  origin_name text,
  package_name text,
  name text,
  plan_path text,
  owner_id bigint,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz,
  visibility text NOT NULL DEFAULT 'public',
  UNIQUE (origin_name, package_name, name)
);
