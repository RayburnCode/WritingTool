-- writiting/migrations/20250417014953_sessions.sql
CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token TEXT NOT NULL,
  user_agent TEXT,
  ip_address INET,
  device_info JSONB,
  is_revoked BOOLEAN NOT NULL DEFAULT false,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create partial unique index for active tokens (replaces the constraint)
CREATE UNIQUE INDEX idx_sessions_unique_active_token ON sessions(token) 
WHERE is_revoked = false;

-- More focused indexes (removed now() calls to avoid immutability issues)
CREATE INDEX idx_sessions_user_id ON sessions(user_id) WHERE is_revoked = false;
CREATE INDEX idx_sessions_expires ON sessions(expires_at) WHERE is_revoked = false;
CREATE INDEX idx_sessions_token ON sessions(token) WHERE is_revoked = false;

-- More efficient trigger
CREATE OR REPLACE FUNCTION update_session_modtime()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW IS DISTINCT FROM OLD THEN
    NEW.updated_at = now();
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_sessions_modtime
  BEFORE UPDATE ON sessions
  FOR EACH ROW
  WHEN (OLD.* IS DISTINCT FROM NEW.*)
  EXECUTE FUNCTION update_session_modtime();