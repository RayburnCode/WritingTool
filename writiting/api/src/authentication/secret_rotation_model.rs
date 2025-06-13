-- 20250417014952_secret_rotation.sql
CREATE TABLE encrypted_secrets (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(100) NOT NULL UNIQUE,  -- e.g., "SMTP_PASSWORD"
  current_value BYTEA NOT NULL,
  previous_value BYTEA,
  encryption_key_id VARCHAR(100) NOT NULL,
  rotated_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_encrypted_secrets_name ON encrypted_secrets(name);