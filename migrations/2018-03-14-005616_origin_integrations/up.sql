CREATE TABLE IF NOT EXISTS origin_integrations (
  id bigserial PRIMARY KEY,
  origin text,
  integration text,
  name text,
  body text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);
