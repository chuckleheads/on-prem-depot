CREATE SEQUENCE IF NOT EXISTS origin_channel_id_seq;

CREATE TABLE IF NOT EXISTS origin_channels (
  id bigint PRIMARY KEY DEFAULT next_id_v1('origin_channel_id_seq'),
  origin_id bigint REFERENCES origins(id),
  owner_id bigint,
  name text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
  UNIQUE(origin_id, name)
);

CREATE TABLE IF NOT EXISTS origin_channel_packages (
  channel_id bigint REFERENCES origin_channels(id) ON DELETE CASCADE,
  package_id bigint REFERENCES origin_packages(id),
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
  PRIMARY KEY (channel_id, package_id)
);