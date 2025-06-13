CREATE TABLE audit_logs (
  id BIGSERIAL PRIMARY KEY,
  user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  action VARCHAR(50) NOT NULL, -- 'login', 'password_change', etc.
  entity_type VARCHAR(50), -- 'user', 'post', etc.
  entity_id UUID, -- Flexible ID (UUID or TEXT)
  ip_address INET,
  user_agent TEXT,
  metadata JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);

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