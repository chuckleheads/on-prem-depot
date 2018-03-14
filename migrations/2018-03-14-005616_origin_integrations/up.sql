CREATE SEQUENCE IF NOT EXISTS origin_integration_id_seq;

CREATE TABLE IF NOT EXISTS origin_integrations (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_integration_id_seq'),
  origin text,
  integration text,
  name text,
  body text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz
);