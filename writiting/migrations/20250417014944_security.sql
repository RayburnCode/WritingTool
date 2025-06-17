-- writiting/migrations/20250417014944_security.sql

-- Password reset tokens (remove duplicate fields from users table)
CREATE TABLE password_reset_tokens (
  token UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  expires_at TIMESTAMPTZ NOT NULL,
  used BOOLEAN NOT NULL DEFAULT false,
  ip_address INET,
  user_agent TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT token_not_expired CHECK (expires_at > created_at)
);

CREATE INDEX idx_password_reset_active ON password_reset_tokens(user_id) 
WHERE used = false;

-- Email verification tokens
CREATE TABLE email_verification_tokens (
  token UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  new_email VARCHAR(255) CHECK (new_email IS NULL OR new_email ~ '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT token_not_expired CHECK (expires_at > created_at)
);

CREATE UNIQUE INDEX idx_email_verification_unique_user ON email_verification_tokens(user_id) 
WHERE new_email IS NULL;

CREATE INDEX idx_email_verification_active ON email_verification_tokens(token);

-- API Keys for programmatic access
CREATE TYPE api_scope AS ENUM ('read', 'write', 'admin');

CREATE TABLE api_keys (
  key UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  scopes api_scope[] NOT NULL,
  allowed_ips CIDR[],
  expires_at TIMESTAMPTZ,
  last_used_at TIMESTAMPTZ,
  last_used_ip INET,
  revoked_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_api_keys_user ON api_keys(user_id) WHERE revoked_at IS NULL;
CREATE INDEX idx_api_keys_active ON api_keys(key) WHERE revoked_at IS NULL;

-- Rate limiting
CREATE TABLE rate_limits (
  bucket VARCHAR(255) PRIMARY KEY,
  tokens DECIMAL NOT NULL CHECK (tokens >= 0),
  capacity DECIMAL NOT NULL CHECK (capacity > 0),
  refill_rate DECIMAL NOT NULL CHECK (refill_rate > 0),
  last_refill TIMESTAMPTZ NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  metadata JSONB DEFAULT '{}'::jsonb
);

CREATE INDEX idx_rate_limit_expires ON rate_limits(expires_at);

-- Security event logging
CREATE TYPE security_action AS ENUM (
  'login_success', 'login_failed', 'logout', 'password_change', 
  'password_reset_request', 'password_reset_complete', 'email_change',
  'api_key_created', 'api_key_revoked', 'suspicious_activity'
);

CREATE TABLE security_logs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  action security_action NOT NULL,
  ip_address INET NOT NULL,
  user_agent TEXT,
  metadata JSONB DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_security_logs_user_id ON security_logs(user_id);
CREATE INDEX idx_security_logs_action ON security_logs(action);
CREATE INDEX idx_security_logs_created_at ON security_logs(created_at);
CREATE INDEX idx_security_logs_ip ON security_logs(ip_address);

-- General audit logging
CREATE TYPE audit_action AS ENUM (
  'create', 'update', 'delete', 'view', 'export', 'import'
);

CREATE TABLE audit_logs (
  id BIGSERIAL PRIMARY KEY,
  user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  action audit_action NOT NULL,
  entity_type VARCHAR(50) NOT NULL,
  entity_id TEXT NOT NULL, -- Using TEXT to handle different ID types
  old_values JSONB,
  new_values JSONB,
  ip_address INET,
  user_agent TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);

-- Account lockout tracking
CREATE TABLE account_lockouts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  reason VARCHAR(100) NOT NULL,
  locked_until TIMESTAMPTZ NOT NULL,
  created_by UUID REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_account_lockouts_user_id ON account_lockouts(user_id);
CREATE INDEX idx_account_lockouts_active ON account_lockouts(user_id);