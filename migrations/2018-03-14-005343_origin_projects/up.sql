CREATE TABLE IF NOT EXISTS origin_projects (
  id PRIMARY KEY SEQUENCE,
  origin_id int REFERENCES origins(id),
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

CREATE TABLE IF NOT EXISTS origin_project_integrations (
  id PRIMARY KEY SEQUENCE,
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
