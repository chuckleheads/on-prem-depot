CREATE SEQUENCE IF NOT EXISTS origin_project_id_seq;

CREATE TABLE IF NOT EXISTS origin_projects (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_project_id_seq'),
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

CREATE SEQUENCE IF NOT EXISTS origin_project_integration_id_seq;

CREATE TABLE IF NOT EXISTS origin_project_integrations (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_project_integration_id_seq'),
  origin text NOT NULL,
  name text NOT NULL,
  integration text NOT NULL,
  integration_name text NOT NULL,
  body text NOT NULL,
  project_id bigint REFERENCES origin_projects(id) ON DELETE CASCADE NOT NULL,
  integration_id bigint REFERENCES origin_integrations(id) ON DELETE CASCADE NOT NULL,  
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
  UNIQUE (project_id, integration_id)
);
