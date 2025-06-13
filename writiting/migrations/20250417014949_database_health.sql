-- 20250417014951_database_health.sql
CREATE TABLE database_connections (
  id BIGSERIAL PRIMARY KEY,
  application_name TEXT NOT NULL,
  connection_count INTEGER NOT NULL,
  max_connections INTEGER NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Run this via cronjob
CREATE OR REPLACE FUNCTION log_connection_stats() RETURNS VOID AS $$
BEGIN
  INSERT INTO database_connections
  SELECT 
    nextval('database_connections_id_seq'),
    application_name,
    count(*),
    (SELECT setting::int FROM pg_settings WHERE name='max_connections'),
    NOW()
  FROM pg_stat_activity 
  GROUP BY application_name;
END;
$$ LANGUAGE plpgsql;