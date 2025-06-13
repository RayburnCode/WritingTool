-- 20250417014953_maintenance.sql
CREATE TABLE maintenance_windows (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  starts_at TIMESTAMPTZ NOT NULL,
  ends_at TIMESTAMPTZ NOT NULL,
  description TEXT NOT NULL,
  is_active BOOLEAN GENERATED ALWAYS AS (
    NOW() BETWEEN starts_at AND ends_at
  ) STORED,
  created_by UUID REFERENCES users(id),
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add to sessions table
ALTER TABLE sessions ADD COLUMN is_maintenance BOOLEAN DEFAULT false;