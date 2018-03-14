CREATE TABLE IF NOT EXISTS origin_channels (
  id PRIMARY KEY SEQUENCE,
  origin_id int REFERENCES origins(id),
  owner_id int,
  name text,
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
  UNIQUE(origin_id, name)
);

CREATE TABLE IF NOT EXISTS origin_channel_packages (
  channel_id int REFERENCES origin_channels(id) ON DELETE CASCADE,
  package_id int REFERENCES origin_packages(id),
  created_at timestamptz DEFAULT now(),
  updated_at timestamptz DEFAULT now(),
  PRIMARY KEY (channel_id, package_id)
);