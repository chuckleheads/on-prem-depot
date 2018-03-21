CREATE TABLE IF NOT EXISTS origin_project_integrations (
  id bigserial PRIMARY KEY,
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
