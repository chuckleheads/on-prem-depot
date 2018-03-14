CREATE TABLE IF NOT EXISTS origin_integrations (
  id PRIMARY KEY SEQUENCE,
  origin text,
  integration text,
  name text,
  body text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);