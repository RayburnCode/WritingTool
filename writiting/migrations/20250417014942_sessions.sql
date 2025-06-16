CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token TEXT NOT NULL,
  user_agent TEXT,
  ip_address INET,  -- Better than TEXT for IP storage
  device_info JSONB, -- Additional device fingerprinting
  is_revoked BOOLEAN NOT NULL DEFAULT false,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT unique_active_token UNIQUE (token) WHERE (is_revoked = false AND expires_at > now())
);

-- Index improvements
CREATE INDEX idx_sessions_token ON sessions(token) WHERE is_revoked = false;
CREATE INDEX idx_sessions_user_id ON sessions(user_id) WHERE is_revoked = false AND expires_at > now();
CREATE INDEX idx_sessions_expires ON sessions(expires_at) WHERE is_revoked = false;

-- Add trigger for automatic updated_at
CREATE OR REPLACE FUNCTION update_session_modtime()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = (now() AT TIME ZONE 'utc');
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_sessions_modtime
BEFORE UPDATE ON sessions
FOR EACH ROW EXECUTE FUNCTION update_session_modtime();