CREATE TABLE feature_flags (
  name VARCHAR(100) PRIMARY KEY,
  is_enabled BOOLEAN DEFAULT false,
  rollout_percentage INT DEFAULT 0, -- 0-100
  target_users UUID[], -- Specific users who get the feature
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);