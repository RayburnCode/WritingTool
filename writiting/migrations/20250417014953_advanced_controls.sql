CREATE TABLE api_keys (
  key UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id),
  name TEXT NOT NULL,
  scopes TEXT[] NOT NULL, -- e.g., {'read', 'write'}
  expires_at TIMESTAMPTZ,
  last_used_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE feature_flags (
  name VARCHAR(100) PRIMARY KEY,
  is_enabled BOOLEAN DEFAULT false,
  rollout_percentage INT DEFAULT 0, -- 0-100
  target_users UUID[], -- Specific users who get the feature
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE rate_limits (
  bucket VARCHAR(255) PRIMARY KEY, -- e.g., "login:192.168.1.1"
  tokens DECIMAL NOT NULL,
  last_refill TIMESTAMPTZ NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL
);