CREATE TABLE password_reset_tokens (
  token UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  expires_at TIMESTAMPTZ NOT NULL,
  used BOOLEAN NOT NULL DEFAULT false,
  ip_address INET,  -- Track originating IP
  user_agent TEXT,  -- Track originating device
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT token_not_expired CHECK (expires_at > created_at)
);

CREATE INDEX idx_password_reset_active ON password_reset_tokens(user_id) 
  WHERE used = false AND expires_at > now();

  CREATE TABLE email_verification_tokens (
  token UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  new_email VARCHAR(255),  -- For email change verification
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT token_not_expired CHECK (expires_at > created_at),
  CONSTRAINT one_token_per_user UNIQUE (user_id) 
    WHERE new_email IS NULL  -- Allow multiple for email changes
);

CREATE INDEX idx_email_verification_active ON email_verification_tokens(token) 
  WHERE expires_at > now();

  CREATE TABLE encrypted_secrets (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(100) NOT NULL,
  environment VARCHAR(50) NOT NULL,  -- 'production', 'staging', etc.
  current_value BYTEA NOT NULL,
  previous_value BYTEA,
  encryption_key_id VARCHAR(100) NOT NULL,
  version INTEGER NOT NULL DEFAULT 1,
  rotated_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT unique_secret_name_env UNIQUE (name, environment)
);

-- Add audit trail for secret rotations
CREATE TABLE secret_rotation_logs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  secret_id UUID NOT NULL REFERENCES encrypted_secrets(id),
  rotation_type VARCHAR(20) NOT NULL,  -- 'manual', 'scheduled', 'emergency'
  rotated_by UUID REFERENCES users(id),  -- If manual rotation
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE TABLE api_keys (
  key UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  scopes TEXT[] NOT NULL,
  allowed_ips CIDR[],  -- IP restrictions
  expires_at TIMESTAMPTZ,
  last_used_at TIMESTAMPTZ,
  last_used_ip INET,
  revoked_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT valid_scopes CHECK (
    scopes <@ ARRAY['read', 'write', 'admin']::TEXT[]  -- Limit to known scopes
  )
);

CREATE INDEX idx_api_keys_user ON api_keys(user_id) WHERE revoked_at IS NULL;
CREATE INDEX idx_api_keys_active ON api_keys(key) WHERE revoked_at IS NULL AND (expires_at IS NULL OR expires_at > now());

CREATE TABLE feature_flags (
  name VARCHAR(100) PRIMARY KEY,
  description TEXT NOT NULL,
  is_enabled BOOLEAN NOT NULL DEFAULT false,
  rollout_percentage INTEGER NOT NULL DEFAULT 0 CHECK (rollout_percentage BETWEEN 0 AND 100),
  target_users UUID[] DEFAULT '{}',
  excluded_users UUID[] DEFAULT '{}',
  target_roles INTEGER[] DEFAULT '{}',  -- References roles.id
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

-- Add audit log for feature flag changes
CREATE TABLE feature_flag_history (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  flag_name VARCHAR(100) NOT NULL,
  changed_by UUID REFERENCES users(id),
  old_value JSONB NOT NULL,
  new_value JSONB NOT NULL,
  changed_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE TABLE rate_limits (
  bucket VARCHAR(255) PRIMARY KEY,
  tokens DECIMAL NOT NULL,
  capacity DECIMAL NOT NULL,
  refill_rate DECIMAL NOT NULL,  -- Tokens per second
  last_refill TIMESTAMPTZ NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  metadata JSONB  -- Additional context like user_id, route, etc.
);

-- Add cleanup index
CREATE INDEX idx_rate_limit_expires ON rate_limits(expires_at);

CREATE TABLE security_logs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id),
  action VARCHAR(50) NOT NULL,  -- 'login', 'password_change', etc.
  ip_address INET NOT NULL,
  user_agent TEXT,
  metadata JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);